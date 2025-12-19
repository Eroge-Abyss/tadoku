<script lang="ts">
  const { children } = $props();
</script>

<div class="info-container">
  <span class="icon-info">
    <i class="fa-solid fa-info-circle"></i>
  </span>
  <p class="note">{@render children()}</p>
</div>

<style>
  .note {
    font-size: 12px;
    color: var(--secondary-text);
    text-align: left;
    margin: 0;
  }
  .info-container {
    display: flex;
    padding: 10px 10px;
    align-items: flex-start;
  }
  .icon-info {
    font-size: 14px;
    margin-right: 5px;
    color: var(--secondary-text);
  }
</style>
