<script lang="ts">
  import type { ProcessResult } from "../api";
  import { saveOutputFile } from "../api";

  interface Props {
    result?: ProcessResult;
    error?: string;
    onreset: () => void;
    onprocessanother?: () => void;
  }

  let { result, error, onreset, onprocessanother }: Props = $props();

  let saving = $state(false);
  let saved = $state(false);
  let saveError = $state("");

  async function handleSave() {
    if (!result) return;
    saving = true;
    saveError = "";
    try {
      const savedPath = await saveOutputFile(
        result.output_path,
        result.output_name,
      );
      if (savedPath) {
        saved = true;
      }
      // Empty string means user cancelled — do nothing
    } catch (e) {
      saveError = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<div class="result-view animate-fade-in-up">
  {#if error}
    <div class="result-card result-error">
      <div class="result-icon-wrap error">
        <svg
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <circle cx="12" cy="12" r="10" />
          <line x1="15" y1="9" x2="9" y2="15" />
          <line x1="9" y1="9" x2="15" y2="15" />
        </svg>
      </div>
      <h3 class="result-title">Processing Failed</h3>
      <p class="result-message mono">{error}</p>
      <button class="btn btn-lg" onclick={onreset} type="button">
        Try Again
      </button>
    </div>
  {:else if result}
    <div class="result-card result-success">
      <div class="result-icon-wrap success">
        <svg
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
          <polyline points="22 4 12 14.01 9 11.01" />
        </svg>
      </div>
      <h3 class="result-title">Processing Complete</h3>

      <div class="result-details">
        <div class="result-row">
          <span class="result-label">File</span>
          <span class="result-value mono">{result.output_name}</span>
        </div>
        <div class="result-row">
          <span class="result-label">Type</span>
          <span class="result-value mono">{result.output_mime}</span>
        </div>
      </div>

      {#if saveError}
        <p class="save-error mono">{saveError}</p>
      {/if}

      <div class="result-actions">
        {#if saved}
          <span class="badge badge-success">✓ Saved</span>
        {:else}
          <button
            class="btn btn-primary btn-lg"
            onclick={handleSave}
            disabled={saving}
            type="button"
          >
            {#if saving}
              Saving…
            {:else}
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
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              Save As
            {/if}
          </button>
        {/if}
        {#if onprocessanother}
          <button class="btn btn-lg" onclick={onprocessanother} type="button">
            Process Another
          </button>
        {/if}
        <button class="btn btn-lg" onclick={onreset} type="button">
          Change Operation
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .result-view {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 300px;
  }

  .result-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-lg);
    padding: var(--space-2xl);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    text-align: center;
    max-width: 400px;
    width: 100%;
  }

  .result-icon-wrap {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 56px;
    height: 56px;
    border-radius: 50%;
  }

  .result-icon-wrap.success {
    background: var(--success-dim);
    color: var(--success);
  }

  .result-icon-wrap.error {
    background: var(--error-dim);
    color: var(--error);
  }

  .result-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--text);
  }

  .result-message {
    font-size: 12px;
    color: var(--error);
    word-break: break-all;
    max-width: 100%;
  }

  .result-details {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
    width: 100%;
    padding: var(--space-md);
    background: var(--bg);
    border-radius: var(--radius-sm);
  }

  .result-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-md);
  }

  .result-label {
    font-size: 12px;
    color: var(--text-3);
    text-transform: uppercase;
    letter-spacing: 0.03em;
    font-weight: 500;
  }

  .result-value {
    font-size: 12px;
    color: var(--text);
  }

  .save-error {
    font-size: 11px;
    color: var(--error);
  }

  .result-actions {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }
</style>
