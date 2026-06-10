<script lang="ts">
  import { settingsStore } from '$lib/stores/settings.svelte';
  import Checkbox from './Checkbox.svelte';

  type Props = {
    categories: string[] | undefined;
    toggleStatus: (_status: string) => void;
    clearStatuses: () => void;
    showUncategorized?: boolean;
  };

  let {
    categories,
    toggleStatus,
    clearStatuses,
    showUncategorized = false,
  }: Props = $props();
</script>

<div class="status-list-container">
  {#each settingsStore.categories as statusItem (statusItem)}
    <label class="menu-item status-checkbox-label">
      {statusItem}
      <Checkbox
        id={`checkbox-${statusItem}`}
        checked={categories ? categories.includes(statusItem) : false}
        onchange={() => toggleStatus(statusItem)}
      />
    </label>
  {/each}
  {#if showUncategorized}
    <label class="menu-item status-checkbox-label">
      Uncategorized
      <Checkbox
        id="checkbox-Uncategorized"
        checked={categories ? categories.includes('Uncategorized') : false}
        onchange={() => toggleStatus('Uncategorized')}
      />
    </label>
  {/if}
</div>
{#if categories && categories.length > 0}
  <div class="menu-divider"></div>
  <button onclick={clearStatuses} class="menu-item danger">
    Clear All
    <i class="fa-regular fa-trash-can"></i>
  </button>
{/if}

<style>
  .status-list-container {
    max-height: 250px;
    overflow-y: auto;
    padding-right: 4px;
  }

  .status-list-container::-webkit-scrollbar {
    width: 6px;
  }

  .status-list-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .status-list-container::-webkit-scrollbar-thumb {
    background: var(--accent);
    border-radius: 4px;
  }

  .status-list-container::-webkit-scrollbar-thumb:hover {
    background: var(--secondary);
  }

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
