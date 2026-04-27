<script lang="ts">
  interface Props {
    title: string;
    description: string;
    icon: string;
    active?: boolean;
    onclick: () => void;
  }

  let { title, description, icon, active = false, onclick }: Props = $props();
</script>

<button class="operation-card" class:active {onclick} type="button">
  <div class="card-icon">{icon}</div>
  <div class="card-content">
    <h3 class="card-title">{title}</h3>
    <p class="card-desc">{description}</p>
  </div>
  <div class="card-arrow">
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
      <path d="M5 12h14M12 5l7 7-7 7" />
    </svg>
  </div>
</button>

<style>
  .operation-card {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    width: 100%;
    padding: var(--space-md) var(--space-lg);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    cursor: pointer;
    text-align: left;
    font-family: var(--font-sans);
    color: var(--text);
    transition:
      background var(--duration-normal) var(--ease-out),
      border-color var(--duration-normal) var(--ease-out),
      box-shadow var(--duration-normal) var(--ease-out),
      transform var(--duration-fast) var(--ease-out);
    outline: none;
  }

  .operation-card:hover {
    background: var(--surface-2);
    border-color: var(--border-hover);
    box-shadow: var(--shadow-sm);
  }

  .operation-card:active {
    transform: scale(0.99);
  }

  .operation-card:focus-visible {
    box-shadow:
      0 0 0 2px var(--bg),
      0 0 0 4px var(--accent-border);
  }

  .operation-card.active {
    border-color: var(--accent-border);
    background: var(--accent-glow);
    box-shadow: var(--shadow-glow);
  }

  .card-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 44px;
    height: 44px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
    font-size: 20px;
    flex-shrink: 0;
    transition: background var(--duration-normal) var(--ease-out);
  }

  .operation-card.active .card-icon {
    background: var(--accent-dim);
  }

  .card-content {
    flex: 1;
    min-width: 0;
  }

  .card-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    margin-bottom: 2px;
  }

  .card-desc {
    font-size: 12px;
    color: var(--text-3);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .operation-card.active .card-title {
    color: var(--accent);
  }

  .card-arrow {
    color: var(--text-3);
    flex-shrink: 0;
    opacity: 0;
    transform: translateX(-4px);
    transition:
      opacity var(--duration-normal) var(--ease-out),
      transform var(--duration-normal) var(--ease-out);
  }

  .operation-card:hover .card-arrow,
  .operation-card.active .card-arrow {
    opacity: 1;
    transform: translateX(0);
  }

  .operation-card.active .card-arrow {
    color: var(--accent);
  }
</style>
