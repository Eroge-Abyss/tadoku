<script>
  export let progress;
  export let color = 'var(--primary)';
</script>

<div class="progress-container">
  <div
    class="progress-bar"
    style="--progress: {progress}%; --bar-color: {color}"
  />
</div>

<style>
  .progress-container {
    width: 100%;
    height: 8px;
    background-color: var(--accent);
    border-radius: var(--small-radius);
    overflow: hidden;
  }

  .progress-bar {
    width: var(--progress);
    height: 100%;
    background-color: var(--bar-color);
    transition: width 0.3s ease;
  }
</style>
