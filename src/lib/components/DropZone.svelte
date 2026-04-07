<script lang="ts">
  import { openFileDialog } from "../api";

  interface Props {
    accept?: string;
    multiple?: boolean;
    onfiles: (paths: string[]) => void;
  }

  let { accept = "", multiple = false, onfiles }: Props = $props();

  let dragging = $state(false);

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    dragging = true;
  }

  function handleDragLeave() {
    dragging = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    dragging = false;
    if (e.dataTransfer?.files) {
      const paths: string[] = [];
      for (const file of e.dataTransfer.files) {
        if ((file as unknown as { path?: string }).path) {
          paths.push((file as unknown as { path: string }).path);
        }
      }
      if (paths.length > 0) {
        onfiles(paths);
      }
    }
  }

  async function handleBrowse() {
    try {
      const extensions = accept
        ? accept.split(",").map((s) => s.trim().replace(".", ""))
        : [];
      const filters = extensions.length
        ? [{ name: "Supported Files", extensions }]
        : [];
      const paths = await openFileDialog(multiple, filters);
      if (paths.length > 0) {
        onfiles(paths);
      }
    } catch (e) {
      alert("Error opening dialog: " + String(e));
    }
  }
</script>

<div
  class="dropzone"
  class:dragging
  role="button"
  tabindex="0"
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  onclick={handleBrowse}
  onkeydown={(e) => e.key === "Enter" && handleBrowse()}
>
  <div class="dropzone-icon">
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
      <polyline points="17 8 12 3 7 8"/>
      <line x1="12" y1="3" x2="12" y2="15"/>
    </svg>
  </div>
  <p class="dropzone-text">
    <span class="dropzone-highlight">Click to browse</span> or drag files here
  </p>
  {#if accept}
    <p class="dropzone-hint mono">{accept}</p>
  {/if}
</div>

<style>
  .dropzone {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-md);
    padding: var(--space-2xl) var(--space-xl);
    border: 2px dashed var(--border);
    border-radius: var(--radius-lg);
    background: var(--surface);
    cursor: pointer;
    transition:
      border-color var(--duration-normal) var(--ease-out),
      background var(--duration-normal) var(--ease-out),
      box-shadow var(--duration-normal) var(--ease-out);
    outline: none;
  }

  .dropzone:hover {
    border-color: var(--border-hover);
    background: var(--surface-2);
  }

  .dropzone:focus-visible {
    box-shadow: 0 0 0 2px var(--bg), 0 0 0 4px var(--accent-border);
  }

  .dropzone.dragging {
    border-color: var(--accent);
    background: var(--accent-glow);
    box-shadow: 0 0 24px var(--accent-glow);
  }

  .dropzone-icon {
    color: var(--text-3);
    transition: color var(--duration-normal) var(--ease-out);
  }

  .dropzone:hover .dropzone-icon {
    color: var(--text-2);
  }

  .dropzone.dragging .dropzone-icon {
    color: var(--accent);
  }

  .dropzone-text {
    font-size: 13px;
    color: var(--text-2);
    text-align: center;
  }

  .dropzone-highlight {
    color: var(--accent);
    font-weight: 500;
  }

  .dropzone-hint {
    font-size: 11px;
    color: var(--text-3);
  }
</style>
