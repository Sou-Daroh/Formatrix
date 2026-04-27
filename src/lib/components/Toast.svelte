<script lang="ts">
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";

  let {
    message,
    type = "info",
    duration = 4000,
    onclose,
  }: {
    message: string;
    type?: "info" | "error" | "success";
    duration?: number;
    onclose: () => void;
  } = $props();

  onMount(() => {
    if (duration > 0) {
      const timer = setTimeout(() => {
        onclose();
      }, duration);
      return () => clearTimeout(timer);
    }
  });
</script>

<div
  class="toast toast-{type}"
  in:fly={{ y: 20, duration: 300 }}
  out:fade={{ duration: 200 }}
  role="alert"
>
  <div class="toast-content">
    {#if type === "error"}
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
        <circle cx="12" cy="12" r="10" />
        <line x1="12" y1="8" x2="12" y2="12" />
        <line x1="12" y1="16" x2="12.01" y2="16" />
      </svg>
    {:else if type === "success"}
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
        <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
        <polyline points="22 4 12 14.01 9 11.01" />
      </svg>
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
        <circle cx="12" cy="12" r="10" />
        <line x1="12" y1="16" x2="12" y2="12" />
        <line x1="12" y1="8" x2="12.01" y2="8" />
      </svg>
    {/if}
    <span>{message}</span>
  </div>
  <button class="toast-close" onclick={onclose} aria-label="Close notification">
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
</div>

<style>
  .toast {
    position: fixed;
    bottom: var(--space-xl);
    right: var(--space-xl);
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-lg);
    z-index: 1000;
    max-width: 400px;
  }

  .toast-info {
    border-left: 3px solid var(--accent);
  }

  .toast-error {
    border-left: 3px solid #ef4444; /* red-500 */
  }

  .toast-success {
    border-left: 3px solid #10b981; /* emerald-500 */
  }

  .toast-content {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
  }

  .toast-info .toast-content svg {
    color: var(--accent);
  }

  .toast-error .toast-content svg {
    color: #ef4444;
  }

  .toast-success .toast-content svg {
    color: #10b981;
  }

  .toast-close {
    background: none;
    border: none;
    padding: var(--space-xs);
    color: var(--text-dim);
    cursor: pointer;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: auto;
    transition: all 0.2s;
  }

  .toast-close:hover {
    background: var(--surface-2);
    color: var(--text);
  }
</style>
