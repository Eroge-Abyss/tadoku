<script>
  // I will make this modular later or maybe not who knows بعد العيد it is.

  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { appState } from '../../routes/state.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let { selected = $bindable() } = $props();
  let searchTerm = $state(''); // Reactive search term
  let isOpen = $state(false); // Reactive dropdown state

  let items = $state();

  onMount(async () => {
    appState.loadGames();
    items = await invoke('get_active_windows');
  });
  // Filtered items based on search term
  let filteredItems = $derived(
    items.filter((item) =>
      item.title.toLowerCase().includes(searchTerm.toLowerCase()),
    ),
  );

  const selectItem = (item) => {
    selected = item;
    searchTerm = item;
    isOpen = false;
  };
</script>

<div class="dropdown">
  <input
    type="text"
    placeholder="Search..."
    bind:value={searchTerm}
    onfocus={() => (isOpen = true)}
  />

  {#if isOpen}
    <div class="dropdown-menu show">
      {#each filteredItems as item}

        <div onclick={() => selectItem(item.exe_path)} class="dropdown-item">
          <img src={item.icon} alt={item.title} width={32} height={32} />
          <span>{item.title}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Hide dropdown when clicking outside -->
<svelte:window
  on:click={(e) => {
    if (!e.target?.closest('.dropdown')) {
      isOpen = false;
    }
  }}
/>

<style>
  .dropdown {
    position: relative;
    width: 300px;
    font-family: Arial, sans-serif;
  }

  input[type='text'] {
    width: 100%;
    padding: 10px;
    font-size: 16px;
    border-radius: 4px;
    background: #313131;
    border: 0;
    margin-top: 1rem;
    margin-bottom: 0.5rem;
    color: var(--main-text);
  }

  .dropdown-menu {
    position: absolute;
    top: 100%;
    left: 0;
    width: 100%;
    max-height: 200px;
    overflow-y: auto;
    border-radius: 4px;
    background: var(--main-background);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    display: none;
  }

  .dropdown-menu.show {
    display: block;
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    padding: 10px;
    cursor: pointer;
    color: var(--main-text);
  }

  .dropdown-item:hover {
    background: var(--main-mauve);
    color: white;
  }

  .dropdown-item img {
    width: 24px;
    height: 24px;
    margin-right: 10px;
  }
</style>
