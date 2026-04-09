use super::{create_temp_dir, PdfSplitOptions, ProcessResult};
use lopdf::{Document, Object, ObjectId};
use std::collections::BTreeMap;
use std::io::Write;

pub fn merge(input_paths: &[String]) -> Result<ProcessResult, String> {
    if input_paths.len() < 2 {
        return Err("at least two pdf files are required for merge".to_string());
    }

    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");
    let mut max_id = 1;

    for path in input_paths {
        let mut doc =
            Document::load(path).map_err(|e| format!("could not loaded pdf {}: {}", path, e))?;
        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc.get_pages()
                .into_values()
                .map(|object_id| (object_id, doc.get_object(object_id).unwrap().clone())),
        );
        documents_objects.extend(doc.objects);
    }

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in documents_objects {
        match object.type_name().unwrap_or("") {
            "Catalog" => {
                catalog_object = Some((
                    catalog_object.map(|(id, _)| id).unwrap_or(object_id),
                    object,
                ));
            }
            "Pages" => {
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref old_object)) = pages_object {
                        if let Ok(old_dictionary) = old_object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }
                    pages_object = Some((
                        pages_object.map(|(id, _)| id).unwrap_or(object_id),
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            "Page" | "Outlines" | "Outline" => {}
            _ => {
                document.objects.insert(object_id, object);
            }
        }
    }

    let pages_parent = pages_object.as_ref().ok_or("Pages root not found")?.0;

    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_parent);
            document
                .objects
                .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    let (catalog_id, catalog_obj) = catalog_object.ok_or("Catalog root not found")?;
    let (page_id, page_obj) = pages_object.unwrap();

    if let Ok(dictionary) = page_obj.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Count", documents_pages.len() as u32);
        dictionary.set(
            "Kids",
            documents_pages
                .into_keys()
                .map(Object::Reference)
                .collect::<Vec<_>>(),
        );
        document
            .objects
            .insert(page_id, Object::Dictionary(dictionary));
    }

    if let Ok(dictionary) = catalog_obj.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", page_id);
        dictionary.remove(b"Outlines");
        document
            .objects
            .insert(catalog_id, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_id);
    document.max_id = document.objects.len() as u32;
    document.renumber_objects();

    let temp_dir = create_temp_dir()?;
    let output_path = temp_dir.join("merged.pdf");
    document
        .save(&output_path)
        .map_err(|e| format!("could not save merged pdf: {}", e))?;

    let output_path_str = output_path
        .to_str()
        .ok_or("could not convert output path to string")?
        .to_string();

    let output_size = std::fs::metadata(&output_path)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(ProcessResult {
        output_path: output_path_str,
        output_name: "merged.pdf".to_string(),
        output_mime: "application/pdf".to_string(),
        output_size,
    })
}

pub fn split(input_path: &str, options: &PdfSplitOptions) -> Result<ProcessResult, String> {
    let doc = Document::load(input_path).map_err(|e| format!("could not open pdf: {}", e))?;

    let pages = doc.get_pages();
    let total_pages = pages.len() as u32;

    let ranges = if options.pages.is_empty() {
        (1..=total_pages)
            .map(|p| (p, p))
            .collect::<Vec<(u32, u32)>>()
    } else {
        parse_page_ranges(&options.pages, total_pages)?
    };

    let temp_dir = create_temp_dir()?;
    let split_dir = temp_dir.join("split");
    std::fs::create_dir_all(&split_dir)
        .map_err(|e| format!("could not create split directory: {}", e))?;

    let mut part_paths: Vec<std::path::PathBuf> = Vec::new();

    for (i, (start, end)) in ranges.iter().enumerate() {
        let mut part_doc = doc.clone();
        let pages_to_remove: Vec<u32> =
            (1..=total_pages).filter(|p| p < start || p > end).collect();

        for &page_num in pages_to_remove.iter().rev() {
            part_doc.delete_pages(&[page_num]);
        }

        let part_name = format!("part_{:02}.pdf", i + 1);
        let part_path = split_dir.join(&part_name);
        part_doc
            .save(&part_path)
            .map_err(|e| format!("could not save split part: {}", e))?;
        part_paths.push(part_path);
    }

    // Create ZIP archive
    let zip_path = temp_dir.join("split.zip");
    let zip_file = std::fs::File::create(&zip_path)
        .map_err(|e| format!("could not create zip file: {}", e))?;
    let mut zip_writer = zip::ZipWriter::new(zip_file);
    let zip_options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for part_path in &part_paths {
        let file_name = part_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("could not get part filename")?;
        zip_writer
            .start_file_from_path(file_name, zip_options)
            .map_err(|e| format!("could not add file to zip: {}", e))?;
        let bytes =
            std::fs::read(part_path).map_err(|e| format!("could not read part file: {}", e))?;
        zip_writer
            .write_all(&bytes)
            .map_err(|e| format!("could not write to zip: {}", e))?;
    }

    zip_writer
        .finish()
        .map_err(|e| format!("could not finalise zip: {}", e))?;

    let output_path_str = zip_path
        .to_str()
        .ok_or("could not convert output path to string")?
        .to_string();

    let output_size = std::fs::metadata(&zip_path).map(|m| m.len()).unwrap_or(0);

    Ok(ProcessResult {
        output_path: output_path_str,
        output_name: "split.zip".to_string(),
        output_mime: "application/zip".to_string(),
        output_size,
    })
}

fn parse_page_ranges(input: &str, total: u32) -> Result<Vec<(u32, u32)>, String> {
    let mut ranges = Vec::new();
    for part in input.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            if bounds.len() != 2 {
                return Err(format!("invalid page range: {}", part));
            }
            let start: u32 = bounds[0]
                .trim()
                .parse()
                .map_err(|_| format!("invalid page number: {}", bounds[0]))?;
            let end: u32 = bounds[1]
                .trim()
                .parse()
                .map_err(|_| format!("invalid page number: {}", bounds[1]))?;
            if start < 1 || end > total || start > end {
                return Err(format!(
                    "page range {}-{} is out of bounds (document has {} pages)",
                    start, end, total
                ));
            }
            ranges.push((start, end));
        } else {
            let page: u32 = part
                .parse()
                .map_err(|_| format!("invalid page number: {}", part))?;
            if page < 1 || page > total {
                return Err(format!(
                    "page {} is out of bounds (document has {} pages)",
                    page, total
                ));
            }
            ranges.push((page, page));
        }
    }
    Ok(ranges)
}
