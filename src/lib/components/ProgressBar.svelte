<script lang="ts">
  interface Props {
    percent: number;
    stage: string;
  }

  let { percent, stage }: Props = $props();
</script>

<div class="progress-container">
  <div class="progress-header">
    <span class="progress-stage">{stage}</span>
    <span class="progress-percent mono">{percent}%</span>
  </div>
  <div class="progress-track">
    <div
      class="progress-bar"
      class:complete={percent >= 100}
      style="width: {Math.min(percent, 100)}%"
    ></div>
  </div>
</div>

<style>
  .progress-container {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .progress-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .progress-stage {
    font-size: 12px;
    color: var(--text-2);
  }

  .progress-percent {
    font-size: 11px;
    color: var(--accent);
  }

  .progress-track {
    width: 100%;
    height: 4px;
    background: var(--border);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-bar {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
    transition: width var(--duration-slow) var(--ease-out);
    animation: progressGlow 2s ease-in-out infinite;
  }

  .progress-bar.complete {
    background: var(--success);
    animation: none;
  }
</style>
