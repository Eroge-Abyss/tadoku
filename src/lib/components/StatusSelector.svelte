<script lang="ts">
  import { GAME_STATUSES } from '$lib/constants';
  let { categories, toggleStatus, clearStatuses } = $props();
  import Checkbox from './Checkbox.svelte';
</script>

{#each GAME_STATUSES as statusItem}
  <label class="menu-item status-checkbox-label">
    {statusItem}
    <Checkbox
      id={`checkbox-${statusItem}`}
      checked={categories?.includes(statusItem)}
      onchange={() => toggleStatus(statusItem)}
    />
  </label>
{/each}
{#if categories && categories.length > 0}
  <div class="menu-divider"></div>
  <button onclick={clearStatuses} class="menu-item danger">
    Clear All
    <i class="fa-regular fa-trash-can"></i>
  </button>
{/if}

<style>
  .menu-item {
    width: 100%;
    border: 0;
    border-radius: var(--small-radius);
    color: var(--main-text);
    background: transparent;
    justify-content: space-between; /* To push chevron to the right */
    padding: 0.75rem 1rem;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    text-align: left;
  }

  .menu-item:hover {
    background: var(--accent);
  }

  .status-checkbox-label {
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .menu-item.danger {
    color: #f7768e;
  }

  .menu-item.danger:hover {
    background: rgba(247, 118, 142, 0.1);
  }

  .menu-divider {
    height: 1px;
    background: var(--accent);
    margin: 0.5rem 0;
  }
</style>
