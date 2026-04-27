import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { readFiles, writeText } from "tauri-plugin-clipboard-next-api";
import { stat, readTextFile } from "@tauri-apps/plugin-fs";

// --- Shared Types ---
export interface ProcessResult {
  output_path: string;
  output_name: string;
  output_mime: string;
  output_size: number;
}

export interface ProgressPayload {
  percent: number;
  stage: string;
}

// --- Processor Options ---
export interface ImageOptions {
  width: number; // 0 = auto
  height: number; // 0 = auto
  quality: number; // 0-100, 0 = default
  format: string; // "jpeg" | "png" | "webp" | "" for same
}

export interface CsvOptions {
  pretty: boolean;
}

export interface PdfSplitOptions {
  pages: string; // e.g. "1,3,5-7"
}

export interface FileFilter {
  name: string;
  extensions: string[];
}

// --- API Functions ---

// File Dialogs
/**
 * Opens a native file dialog.
 */
export async function openFileDialog(
  multiple: boolean,
  filters: FileFilter[],
): Promise<string[]> {
  const result = await invoke<string[]>("open_file_dialog", {
    multiple,
    filters,
  });
  return result;
}

export async function saveOutputFile(
  tempPathStr: string,
  suggestedName: string,
): Promise<string> {
  return invoke<string>("save_output_file", { tempPathStr, suggestedName });
}

export async function getFileSize(path: string): Promise<number> {
  try {
    const fileStat = await stat(path);
    return fileStat.size || 0;
  } catch {
    return 0;
  }
}

// Processors
export async function processImage(
  inputPath: string,
  options: ImageOptions,
): Promise<ProcessResult> {
  return invoke<ProcessResult>("process_image", { inputPath, options });
}

export async function processImageBatch(
  inputPaths: string[],
  options: ImageOptions,
): Promise<ProcessResult> {
  return invoke<ProcessResult>("process_image_batch", { inputPaths, options });
}

export async function processCsvJson(
  inputPath: string,
  options: CsvOptions,
): Promise<ProcessResult> {
  return invoke<ProcessResult>("process_csv_json", { inputPath, options });
}

export async function processPdfText(
  inputPath: string,
): Promise<ProcessResult> {
  return invoke<ProcessResult>("process_pdf_text", { inputPath });
}

export async function processPdfMerge(
  inputPaths: string[],
): Promise<ProcessResult> {
  return invoke<ProcessResult>("process_pdf_merge", { inputPaths });
}

export async function processPdfSplit(
  inputPath: string,
  options: PdfSplitOptions,
): Promise<ProcessResult> {
  return invoke<ProcessResult>("process_pdf_split", { inputPath, options });
}

// Events
export async function listenToProgress(
  callback: (payload: ProgressPayload) => void,
): Promise<UnlistenFn> {
  return await listen<ProgressPayload>("progress", (event) => {
    callback(event.payload);
  });
}

/**
 * Listens for native OS file drop events on the Tauri window.
 * Returns the absolute paths of the dropped files.
 */
export async function listenForFileDrop(
  callback: (paths: string[]) => void,
): Promise<UnlistenFn> {
  // Tauri 2: only onDragDropEvent fires for native OS drops
  const unlisten = await getCurrentWebviewWindow().onDragDropEvent((event) => {
    if (event.payload.type === "drop") {
      callback(event.payload.paths);
    }
  });

  return unlisten;
}

/**
 * Reads file paths copied to the system clipboard (Ctrl+V).
 */
export async function getFilesFromClipboard(): Promise<string[]> {
  try {
    const result = await readFiles();
    if (!result || !result.files || result.files.length === 0) return [];

    // Normalize potential URI paths (file:///C:/...)
    return result.files.map((fileItem) => {
      let path = fileItem.path;
      if (path.startsWith("file://")) {
        path = decodeURI(path.replace("file://", ""));
        // Handle windows drive letter `/C:/` -> `C:/`
        if (path.match(/^\/[A-Za-z]:\//)) {
          path = path.slice(1);
        }
      }
      return path;
    });
  } catch (e) {
    console.warn("Failed to read files from clipboard:", e);
    return [];
  }
}

// Preview helpers
export function getPreviewImageUrl(filePath: string): string {
  return convertFileSrc(filePath);
}

export async function getPreviewText(filePath: string): Promise<string> {
  try {
    const content = await readTextFile(filePath);
    return content;
  } catch {
    return "";
  }
}

export async function copyTextFromResult(tempPathStr: string): Promise<void> {
  const text = await readTextFile(tempPathStr);
  await writeText(text);
}
