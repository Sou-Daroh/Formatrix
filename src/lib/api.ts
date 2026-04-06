import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

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
  width?: number | null;
  height?: number | null;
  quality: number; // 0-100
  format?: string | null;
}

export interface CsvOptions {
  delimiter: string;
  has_headers: boolean;
}

export interface PdfSplitOptions {
  pages: string; // e.g. "1,3,5-7"
}

// --- API Functions ---

// File Dialogs
export async function openFileDialog(
  multiple: boolean = false,
  filters: Array<[string, string[]]> = [],
): Promise<string[]> {
  const formattedFilters = filters.map(([name, exts]) => ({
    name,
    extensions: exts,
  }));
  // Wrap into correct tuple expected by backend args
  return invoke<string[]>("open_file_dialog", {
    multiple,
    filters: formattedFilters,
  });
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
) {
  return listen<ProgressPayload>("progress", (event) => {
    callback(event.payload);
  });
}
