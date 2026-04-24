<script lang="ts">
  import { formatSize } from "../utils";

  interface Props {
    files: string[];
    sizes?: Record<string, number>;
    onremove: (index: number) => void;
  }

  let { files, sizes = {}, onremove }: Props = $props();

  function basename(path: string): string {
    return path.split(/[\\/]/).pop() || path;
  }
</script>

{#if files.length > 0}
  <div class="file-list">
    <div class="file-list-header">
      <span class="label">Staged Files</span>
      <span class="badge">{files.length}</span>
    </div>
    <ul class="file-items">
      {#each files as file, i}
        <li
          class="file-item animate-fade-in-up"
          style="animation-delay: {i * 40}ms"
        >
          <svg
            class="file-icon"
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path
              d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"
            />
            <polyline points="14 2 14 8 20 8" />
          </svg>
          <div class="file-details">
            <span class="file-name mono" title={file}>{basename(file)}</span>
            {#if sizes[file] !== undefined}
              <span class="file-size mono text-dim"
                >{formatSize(sizes[file])}</span
              >
            {/if}
          </div>
          <button
            class="file-remove"
            onclick={() => onremove(i)}
            type="button"
            aria-label="Remove file"
          >
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
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>
        </li>
      {/each}
    </ul>
  </div>
{/if}

<style>
  .file-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .file-list-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .file-items {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-height: 160px;
    overflow-y: auto;
  }

  .file-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) 12px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    transition: border-color var(--duration-fast) var(--ease-out);
  }

  .file-item:hover {
    border-color: var(--border-hover);
  }

  .file-icon {
    color: var(--text-3);
    flex-shrink: 0;
  }

  .file-details {
    flex: 1;
    display: flex;
    justify-content: space-between;
    align-items: center;
    min-width: 0;
    gap: 12px;
  }

  .file-name {
    font-size: 12px;
    color: var(--text-2);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-size {
    font-size: 11px;
    flex-shrink: 0;
  }

  .file-remove {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-3);
    cursor: pointer;
    flex-shrink: 0;
    transition:
      background var(--duration-fast) var(--ease-out),
      color var(--duration-fast) var(--ease-out);
  }

  .file-remove:hover {
    background: var(--error-dim);
    color: var(--error);
  }
</style>
