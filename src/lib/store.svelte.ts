/* global $state */
import type { ProcessResult } from "./api";

export type TaskStatus = "idle" | "processing" | "completed" | "error";
export type TaskType =
  | "image"
  | "csv_json"
  | "pdf_text"
  | "pdf_merge"
  | "pdf_split";

export interface ProcessTask {
  id: string; // unique ID
  type: TaskType;
  inputs: string[];
  status: TaskStatus;
  progress: number;
  stage: string;
  result?: ProcessResult;
  error?: string;
  options?: Record<string, unknown>;
}

// Internal reactive state wrapper
export function createProcessingStore() {
  let tasks = $state<ProcessTask[]>([]);

  return {
    get tasks() {
      return tasks;
    },
    addTask: (
      task: Omit<ProcessTask, "id" | "status" | "progress" | "stage">,
    ) => {
      const id = crypto.randomUUID();
      tasks.push({
        ...task,
        id,
        status: "idle",
        progress: 0,
        stage: "Waiting...",
      });
      return id;
    },
    updateTaskStatus: (id: string, status: TaskStatus) => {
      const task = tasks.find((t) => t.id === id);
      if (task) task.status = status;
    },
    updateTaskProgress: (id: string, progress: number, stage: string) => {
      const task = tasks.find((t) => t.id === id);
      if (task) {
        task.progress = progress;
        task.stage = stage;
      }
    },
    setTaskResult: (id: string, result: ProcessResult) => {
      const task = tasks.find((t) => t.id === id);
      if (task) {
        task.result = result;
        task.status = "completed";
        task.progress = 100;
        task.stage = "Completed";
      }
    },
    setTaskError: (id: string, error: string) => {
      const task = tasks.find((t) => t.id === id);
      if (task) {
        task.error = error;
        task.status = "error";
        task.stage = "Failed";
      }
    },
    removeTask: (id: string) => {
      tasks = tasks.filter((t) => t.id !== id);
    },
    clearCompleted: () => {
      tasks = tasks.filter(
        (t) => t.status !== "completed" && t.status !== "error",
      );
    },
  };
}

// Global default export
export const processingStore = createProcessingStore();
