use super::{create_temp_dir, ProcessResult};
use lopdf::{Document, Object, ObjectId};
use std::collections::BTreeMap;

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

    let first_stem = std::path::Path::new(&input_paths[0])
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    Ok(ProcessResult {
        output_path: output_path_str,
        output_name: format!("{}_merged.pdf", first_stem),
        output_mime: "application/pdf".to_string(),
        output_size,
    })
}
