import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

// --- Shared Types ---
export interface ProcessResult {
  output_path: string;
  output_name: string;
  output_mime: string;
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

// Processors
export async function processImage(
  inputPath: string,
  options: ImageOptions,
): Promise<ProcessResult> {
  return invoke<ProcessResult>("process_image", { inputPath, options });
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
  const unlisten1 = await getCurrentWebviewWindow().onDragDropEvent((event) => {
    if (event.payload.type === "drop") {
      callback(event.payload.paths);
    }
  });

  const unlisten2 = await listen<{ paths: string[] }>(
    "tauri://drag-drop",
    (event) => {
      if (event.payload && event.payload.paths) callback(event.payload.paths);
    },
  );

  const unlisten3 = await listen<{ paths: string[] }>(
    "tauri://drop",
    (event) => {
      if (event.payload && event.payload.paths) callback(event.payload.paths);
    },
  );

  const unlisten4 = await listen<{ paths: string[] }>(
    "tauri://file-drop",
    (event) => {
      if (event.payload && event.payload.paths) callback(event.payload.paths);
    },
  );

  return () => {
    unlisten1();
    unlisten2();
    unlisten3();
    unlisten4();
  };
}
