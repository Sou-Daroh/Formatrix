<script lang="ts">
  import OperationCard from "./lib/components/OperationCard.svelte";
  import DropZone from "./lib/components/DropZone.svelte";
  import FileList from "./lib/components/FileList.svelte";
  import OptionsPanel from "./lib/components/OptionsPanel.svelte";
  import ProgressBar from "./lib/components/ProgressBar.svelte";
  import ResultView from "./lib/components/ResultView.svelte";
  import Toast from "./lib/components/Toast.svelte";
  import {
    processImage,
    processCsvJson,
    processPdfText,
    processPdfMerge,
    processPdfSplit,
    listenToProgress,
    getFileSize,
  } from "./lib/api";
  import type {
    ProcessResult,
    ImageOptions,
    CsvOptions,
    PdfSplitOptions,
    ProgressPayload,
  } from "./lib/api";
  import type { TaskType } from "./lib/types";

  // --- App State ---
  type AppStep = "choose" | "configure" | "processing" | "result";

  let step = $state<AppStep>("choose");
  let selectedOp = $state<TaskType | null>(null);
  let files = $state<string[]>([]);
  let fileSizes = $state<Record<string, number>>({});
  let progress = $state(0);
  let progressStage = $state("Preparing...");
  let result = $state<ProcessResult | undefined>(undefined);
  let error = $state("");
  let isProcessing = $state(false);

  // --- Processing Time State ---
  let elapsedTime = $state(0);
  let timerInterval: ReturnType<typeof setInterval> | undefined;

  function formatTime(seconds: number) {
    const m = Math.floor(seconds / 60);
    const s = seconds % 60;
    return `${m}:${s.toString().padStart(2, "0")}`;
  }

  // --- Toast State ---
  let toastMessage = $state("");
  let toastType = $state<"info" | "error" | "success">("info");

  function showToast(msg: string, type: "info" | "error" | "success" = "info") {
    toastMessage = msg;
    toastType = type;
  }

  function closeToast() {
    toastMessage = "";
  }

  // --- Options State ---
  let imageOpts = $state<ImageOptions>({
    width: 0,
    height: 0,
    quality: 0,
    format: "",
  });
  let csvOpts = $state<CsvOptions>({ pretty: true });
  let splitOpts = $state<PdfSplitOptions>({ pages: "" });

  // --- Operations Definition ---
  const operations: Array<{
    type: TaskType;
    title: string;
    description: string;
    icon: string;
    accept: string;
    hint: string;
    multiple: boolean;
  }> = [
    {
      type: "image",
      title: "Image Resize",
      description: "Resize, compress, and convert images",
      icon: "🖼",
      accept: ".jpg,.jpeg,.png,.webp,.bmp,.gif",
      hint: "Supports JPG, PNG, WebP, GIF, BMP",
      multiple: false,
    },
    {
      type: "csv_json",
      title: "CSV → JSON",
      description: "Convert CSV spreadsheets to JSON",
      icon: "📊",
      accept: ".csv",
      hint: "CSV files only",
      multiple: false,
    },
    {
      type: "pdf_text",
      title: "PDF Text Extract",
      description: "Pull selectable text from PDF documents",
      icon: "📄",
      accept: ".pdf",
      hint: "PDF files only",
      multiple: false,
    },
    {
      type: "pdf_merge",
      title: "PDF Merge",
      description: "Combine multiple PDFs into one document",
      icon: "📑",
      accept: ".pdf",
      hint: "Select 2 or more PDF files",
      multiple: true,
    },
    {
      type: "pdf_split",
      title: "PDF Split",
      description: "Split a PDF by page ranges into a ZIP",
      icon: "✂️",
      accept: ".pdf",
      hint: "PDF files only",
      multiple: false,
    },
  ];

  // --- Derived ---
  let currentOp = $derived(operations.find((o) => o.type === selectedOp));
  let isValidPdfSplitPages = $derived(
    selectedOp === "pdf_split"
      ? splitOpts.pages.trim() === "" ||
          /^(\s*\d+\s*(-\s*\d+\s*)?(,\s*\d+\s*(-\s*\d+\s*)?)*)?$/.test(
            splitOpts.pages,
          )
      : true,
  );

  let isValidImageOpts = $derived(
    selectedOp === "image"
      ? !(
          imageOpts.width === 0 &&
          imageOpts.height === 0 &&
          imageOpts.format === ""
        )
      : true,
  );

  let canProcess = $derived(
    (selectedOp === "pdf_merge" ? files.length >= 2 : files.length >= 1) &&
      isValidPdfSplitPages &&
      isValidImageOpts,
  );

  let windowTitle = $derived(
    step === "processing"
      ? `Processing... - Formatrix`
      : currentOp
        ? `${currentOp.title} - Formatrix`
        : `Formatrix`,
  );

  // --- Handlers ---
  function selectOperation(type: TaskType) {
    selectedOp = type;
    files = [];
    result = undefined;
    error = "";
    progress = 0;
    progressStage = "Preparing...";
    imageOpts = { width: 0, height: 0, quality: 0, format: "" };
    csvOpts = { pretty: true };
    splitOpts = { pages: "" };
    step = "configure";
  }

  async function handleFiles(paths: string[]) {
    const maxBytes = 1024 * 1024 * 1024; // 1 GB limit
    let validPaths: string[] = [];
    let rejectedCount = 0;

    for (const p of paths) {
      const size = await getFileSize(p);
      if (size > maxBytes) {
        rejectedCount++;
      } else {
        validPaths.push(p);
        fileSizes[p] = size;
      }
    }

    if (rejectedCount > 0) {
      showToast(
        `Skipped ${rejectedCount} file(s) because they exceed the 1 GB limit.`,
        "error",
      );
    }

    if (validPaths.length === 0) return;

    if (currentOp?.multiple) {
      // Deduplicate: only add paths not already in the queue
      const newPaths = validPaths.filter((p) => !files.includes(p));
      files = [...files, ...newPaths];
    } else {
      files = validPaths.slice(0, 1);
    }
  }

  function removeFile(index: number) {
    const path = files[index];
    files = files.filter((_, i) => i !== index);
    const newSizes = { ...fileSizes };
    delete newSizes[path];
    fileSizes = newSizes;
  }

  function goBack() {
    if (files.length > 0) {
      if (
        !confirm("You have files staged. Are you sure you want to go back?")
      ) {
        return;
      }
    }
    step = "choose";
    selectedOp = null;
    files = [];
    fileSizes = {};
  }

  function handleKeydown(e: KeyboardEvent) {
    if (
      e.key === "Enter" &&
      (e.ctrlKey || e.metaKey) &&
      step === "configure" &&
      canProcess
    ) {
      e.preventDefault();
      handleProcess();
    }
    if (e.key === "Escape" && step !== "choose" && step !== "processing") {
      e.preventDefault();
      if (step === "result") {
        resetAll();
      } else {
        goBack();
      }
    }
  }

  async function handleProcess() {
    if (!selectedOp || !canProcess || isProcessing) return;
    isProcessing = true;
    step = "processing";
    progress = 10;
    progressStage = "Starting...";
    result = undefined;
    error = "";
    elapsedTime = 0;

    timerInterval = setInterval(() => {
      elapsedTime++;
    }, 1000);

    const unlisten = await listenToProgress((p: ProgressPayload) => {
      progress = p.percent;
      progressStage = p.stage;
    });

    try {
      let res: ProcessResult;
      switch (selectedOp) {
        case "image":
          res = await processImage(files[0], imageOpts);
          break;
        case "csv_json":
          res = await processCsvJson(files[0], csvOpts);
          break;
        case "pdf_text":
          res = await processPdfText(files[0]);
          break;
        case "pdf_merge":
          res = await processPdfMerge(files);
          break;
        case "pdf_split":
          res = await processPdfSplit(files[0], splitOpts);
          break;
      }
      result = res;
      progress = 100;
      progressStage = "Complete";
    } catch (e) {
      error = String(e);
    } finally {
      if (timerInterval) {
        clearInterval(timerInterval);
        timerInterval = undefined;
      }
      unlisten();
      isProcessing = false;
      step = "result";
    }
  }

  function resetAll() {
    step = "choose";
    selectedOp = null;
    files = [];
    fileSizes = {};
    result = undefined;
    error = "";
    progress = 0;
  }

  function processAnother() {
    files = [];
    fileSizes = {};
    result = undefined;
    error = "";
    progress = 0;
    progressStage = "Preparing...";
    step = "configure";
  }
  function handleGlobalError(e: Event) {
    const errorEvent = e as ErrorEvent;
    console.error(
      "Global error caught:",
      errorEvent.error || errorEvent.message,
    );
    // If an error happens outside processing, we at least show it to the user
    // rather than failing silently.
    if (step !== "processing") {
      error = String(errorEvent.error || errorEvent.message || "Unknown error");
      step = "result";
    } else {
      showToast(
        String(errorEvent.error || errorEvent.message || "Unknown error"),
        "error",
      );
    }
  }

  function handleUnhandledRejection(e: PromiseRejectionEvent) {
    console.error("Unhandled promise rejection:", e.reason);
    if (step !== "processing") {
      error = String(e.reason);
      step = "result";
    } else {
      showToast(String(e.reason), "error");
    }
  }
</script>

<svelte:window
  onkeydown={handleKeydown}
  onerror={handleGlobalError}
  onunhandledrejection={handleUnhandledRejection}
/>

<svelte:head>
  <title>{windowTitle}</title>
</svelte:head>

{#if toastMessage}
  <Toast message={toastMessage} type={toastType} onclose={closeToast} />
{/if}

<div class="app-shell">
  <!-- Header -->
  <header class="app-header">
    <div class="header-left">
      {#if step !== "choose" && step !== "processing"}
        <button class="btn btn-ghost btn-sm" onclick={goBack} type="button">
          <svg
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="19" y1="12" x2="5" y2="12" />
            <polyline points="12 19 5 12 12 5" />
          </svg>
          Back
        </button>
      {/if}
    </div>
    <div class="header-brand">
      <span class="brand-mark">◆</span>
      <span class="brand-name">Formatrix</span>
    </div>
    <div class="header-right">
      {#if step === "configure" && currentOp}
        <span class="badge">{currentOp.title}</span>
      {/if}
    </div>
  </header>

  <!-- Main Content -->
  <main class="app-main">
    {#key step}
      <div class="step-container animate-fade-in">
        <!-- Step 1: Choose Operation -->
        {#if step === "choose"}
          <div class="choose-view">
            <div class="choose-header">
              <h1 class="choose-title">What do you need?</h1>
              <p class="choose-subtitle text-muted">
                Choose an operation to get started. All processing happens
                locally.
              </p>
            </div>
            <div class="operation-grid stagger">
              {#each operations as op}
                <OperationCard
                  title={op.title}
                  description={op.description}
                  icon={op.icon}
                  active={selectedOp === op.type}
                  onclick={() => selectOperation(op.type)}
                />
              {/each}
            </div>
          </div>

          <!-- Step 2: Configure & Drop Files -->
        {:else if step === "configure" && currentOp}
          <div class="configure-view">
            <div class="configure-layout">
              <!-- Left: Files -->
              <div class="configure-files">
                <DropZone
                  accept={currentOp.accept}
                  hint={currentOp.hint}
                  multiple={currentOp.multiple}
                  onfiles={handleFiles}
                />
                <FileList {files} sizes={fileSizes} onremove={removeFile} />
              </div>
              <!-- Right: Options -->
              <div class="configure-options">
                <div class="options-header">
                  <h2 class="options-title">Options</h2>
                </div>
                <OptionsPanel
                  taskType={currentOp.type}
                  imageOptions={imageOpts}
                  csvOptions={csvOpts}
                  pdfSplitOptions={splitOpts}
                />
              </div>
            </div>
            <div class="configure-footer">
              {#if !isValidPdfSplitPages}
                <div class="validation-error">
                  Invalid page range format. Use numbers or ranges (e.g.
                  1,3,5-7).
                </div>
              {:else if !isValidImageOpts}
                <div class="validation-error">
                  Image operation is a no-op (no size or format changes).
                </div>
              {/if}
              <button
                class="btn btn-primary btn-lg process-btn"
                disabled={!canProcess}
                onclick={handleProcess}
                type="button"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <polygon points="5 3 19 12 5 21 5 3" />
                </svg>
                Process{#if selectedOp === "pdf_merge"}
                  ({files.length} files){/if}
              </button>
              <span class="shortcut-hint mono text-dim">Ctrl + Enter</span>
            </div>
          </div>

          <!-- Step 3: Processing -->
        {:else if step === "processing"}
          <div class="processing-view">
            <div class="processing-card">
              <div class="processing-spinner"></div>
              <h2 class="processing-title">Processing…</h2>
              <ProgressBar percent={progress} stage={progressStage} />
              <div class="processing-time mono text-dim">
                Elapsed: {formatTime(elapsedTime)}
              </div>
            </div>
          </div>

          <!-- Step 4: Result -->
        {:else if step === "result"}
          <ResultView
            {result}
            {error}
            onreset={resetAll}
            onprocessanother={processAnother}
          />
        {/if}
      </div>
    {/key}
  </main>

  <!-- Footer -->
  <footer class="app-footer">
    <span class="footer-text mono text-dim"
      >v{__APP_VERSION__} · All processing is offline</span
    >
  </footer>
</div>

<style>
  /* --- Shell --- */
  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  /* --- Header --- */
  .app-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm) var(--space-lg);
    height: 48px;
    border-bottom: 1px solid var(--border);
    background: var(--surface);
    flex-shrink: 0;
    -webkit-app-region: drag;
  }

  .header-left,
  .header-right {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    -webkit-app-region: no-drag;
  }

  .header-right {
    justify-content: flex-end;
    white-space: nowrap;
  }

  .header-brand {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .brand-mark {
    color: var(--accent);
    font-size: 14px;
  }

  .brand-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    letter-spacing: 0.04em;
  }

  /* --- Main --- */
  .app-main {
    flex: 1;
    overflow-y: auto;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-xl);
  }

  .step-container {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

  /* --- Footer --- */
  .app-footer {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-sm);
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }

  .footer-text {
    font-size: 11px;
  }

  /* --- Choose View --- */
  .choose-view {
    display: flex;
    flex-direction: column;
    gap: var(--space-xl);
    max-width: 480px;
    width: 100%;
  }

  .choose-header {
    text-align: center;
  }

  .choose-title {
    font-size: 24px;
    font-weight: 600;
    color: var(--text);
    letter-spacing: -0.02em;
    margin-bottom: var(--space-sm);
  }

  .choose-subtitle {
    font-size: 13px;
  }

  .operation-grid {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  /* --- Configure View --- */
  .configure-view {
    display: flex;
    flex-direction: column;
    gap: var(--space-xl);
    width: 100%;
    max-width: 720px;
  }

  .configure-layout {
    display: grid;
    grid-template-columns: 1fr 280px;
    gap: var(--space-xl);
    align-items: start;
  }

  .configure-files {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .configure-options {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
    padding: var(--space-lg);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
  }

  .options-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-sm);
  }

  .options-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .configure-footer {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-sm);
  }

  .validation-error {
    color: #ef4444; /* red-500 */
    font-size: 12px;
    font-weight: 500;
    text-align: center;
    margin-bottom: var(--space-xs);
  }

  .process-btn {
    min-width: 200px;
  }

  .process-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    transform: none;
  }

  .shortcut-hint {
    font-size: 11px;
  }

  /* --- Processing View --- */
  .processing-view {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .processing-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-lg);
    padding: var(--space-2xl);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    min-width: 320px;
  }

  .processing-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .processing-title {
    font-size: 16px;
    font-weight: 600;
    color: var(--text);
  }

  /* --- Responsive Breakpoints --- */
  @media (max-width: 640px) {
    .configure-layout {
      grid-template-columns: 1fr;
    }

    .app-header {
      padding: var(--space-sm) var(--space-md);
    }
  }
</style>
