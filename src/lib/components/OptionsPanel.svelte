<script lang="ts">
  import type { TaskType } from "../store.svelte";
  import type { ImageOptions, CsvOptions, PdfSplitOptions } from "../api";

  interface Props {
    taskType: TaskType;
    imageOptions: ImageOptions;
    csvOptions: CsvOptions;
    pdfSplitOptions: PdfSplitOptions;
  }

  let { taskType, imageOptions, csvOptions, pdfSplitOptions }: Props = $props();
</script>

<div class="options-panel">
  {#if taskType === "image"}
    <div class="option-group">
      <label class="label" for="opt-width">Width (px)</label>
      <input
        id="opt-width"
        class="input"
        type="number"
        min="0"
        max="10000"
        placeholder="0 = auto"
        bind:value={imageOptions.width}
      />
    </div>
    <div class="option-group">
      <label class="label" for="opt-height">Height (px)</label>
      <input
        id="opt-height"
        class="input"
        type="number"
        min="0"
        max="10000"
        placeholder="0 = auto"
        bind:value={imageOptions.height}
      />
    </div>
    <div class="option-group">
      <label class="label" for="opt-quality">
        Quality
        <span class="option-value mono">{imageOptions.quality || 'auto'}</span>
      </label>
      <input
        id="opt-quality"
        type="range"
        min="0"
        max="100"
        step="1"
        bind:value={imageOptions.quality}
      />
    </div>
    <div class="option-group">
      <label class="label" for="opt-format">Output Format</label>
      <select id="opt-format" class="select" bind:value={imageOptions.format}>
        <option value="">Same as input</option>
        <option value="jpeg">JPEG</option>
        <option value="png">PNG</option>
        <option value="webp">WebP</option>
      </select>
    </div>

  {:else if taskType === "csv_json"}
    <div class="option-group">
      <label class="toggle">
        <input type="checkbox" bind:checked={csvOptions.pretty} />
        Pretty-print JSON output
      </label>
    </div>

  {:else if taskType === "pdf_split"}
    <div class="option-group">
      <label class="label" for="opt-pages">Page Ranges</label>
      <input
        id="opt-pages"
        class="input"
        type="text"
        placeholder="e.g. 1,3,5-7 (empty = all pages)"
        bind:value={pdfSplitOptions.pages}
      />
      <p class="option-hint">Comma-separated page numbers or ranges. Leave empty to split every page.</p>
    </div>

  {:else if taskType === "pdf_text"}
    <div class="option-info">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="10"/>
        <line x1="12" y1="16" x2="12" y2="12"/>
        <line x1="12" y1="8" x2="12.01" y2="8"/>
      </svg>
      <span>Extracts all selectable text from the PDF. No options needed.</span>
    </div>

  {:else if taskType === "pdf_merge"}
    <div class="option-info">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="10"/>
        <line x1="12" y1="16" x2="12" y2="12"/>
        <line x1="12" y1="8" x2="12.01" y2="8"/>
      </svg>
      <span>Select two or more PDFs to merge into a single document.</span>
    </div>
  {/if}
</div>

<style>
  .options-panel {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .option-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .option-value {
    float: right;
    font-size: 11px;
    color: var(--accent);
  }

  .option-hint {
    font-size: 11px;
    color: var(--text-3);
    margin-top: 2px;
  }

  .option-info {
    display: flex;
    align-items: flex-start;
    gap: var(--space-sm);
    padding: var(--space-md);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    font-size: 12px;
    color: var(--text-2);
    line-height: 1.5;
  }

  .option-info svg {
    flex-shrink: 0;
    color: var(--text-3);
    margin-top: 1px;
  }
</style>
