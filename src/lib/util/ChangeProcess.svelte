<script lang="ts">
  import type { Process } from '$lib/types';
  import Dialog from '$lib/util/Dialog.svelte';
  import { appState } from '../../routes/state.svelte';

  let {
    isOpen = $bindable(),
    gameId,
    processList,
    title = 'Select game process',
  } = $props();

  let isDropdownOpen = $state(false);
  let loading = $state(false);
  let process = $state<Process | null>(null);
  let searchTerm = $state(''); // Reactive search term

  async function onConfirm(selectedProcessPath: string) {
    appState.updateGameProcessPath(gameId, selectedProcessPath);

    process = null;
    searchTerm = '';
  }

  // Close the modal
  function closeModal() {
    isOpen = false;
  }

  // Handle the "OK" button click
  function handleConfirm() {
    if (process) {
      onConfirm(process.exe_path); // Run the provided function
    }
    closeModal(); // Close the modal
  }

  // Filtered items based on search term
  let filteredItems = $derived(
    processList.filter((item: Process) =>
      item.title.toLowerCase().includes(searchTerm.toLowerCase()),
    ),
  );

  const selectItem = (item: Process) => {
    process = item;
    searchTerm = item.title;
    isDropdownOpen = false;
  };
</script>

<section>
  <Dialog show={isOpen} close={closeModal}>
    {#snippet header()}
      {title}
    {/snippet}

    <section class="game-form">
      <div class="dropdown">
        <input
          type="text"
          placeholder="Search..."
          bind:value={searchTerm}
          onfocus={() => (isDropdownOpen = true)}
        />

        {#if isDropdownOpen}
          <div class="dropdown-menu show">
            {#each filteredItems as item}
              <div onclick={() => selectItem(item)} class="dropdown-item">
                <img src={item.icon} alt={item.title} width={32} height={32} />
                <span>{item.title}</span>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <div class="info-container">
        <span class="icon-info">
          <i class="fa-solid fa-info-circle"></i>
        </span>
        <p class="note">
          If you can't find your game, try setting it to windowed then return to
          full screen when done.
        </p>
      </div>

      <button disabled={loading} class="save-button" onclick={handleConfirm}>
        {#if loading}
          Saving...
        {:else}
          Save
        {/if}
      </button>
    </section>
  </Dialog>
</section>

<!-- Hide dropdown when clicking outside -->
<svelte:window
  on:click={(e) => {
    // @ts-ignore
    if (!e.target?.closest('.dropdown')) {
      isDropdownOpen = false;
    }
  }}
/>

<style>
  .game-form {
    margin: 1rem;
  }

  .game-form button {
    border: 0;
    background-color: #313131;
    border-radius: var(--small-radius);
    color: #fff;
    width: 100%;
    padding: 0.5rem;
    font-size: 18px;
    margin-top: 1rem;
    cursor: pointer;
    transition: background-color 0.3s ease;

    &:hover {
      background-color: #404040;
    }
  }

  .save-button {
    background: var(--primary) !important;
    &[disabled] {
      opacity: 0.5;
    }
    &:hover:not([disabled]) {
      background: var(
        --primary-dark,
        color-mix(in srgb, var(--primary), #000 10%)
      ) !important;
    }
  }

  .dropdown {
    position: relative;
    font-family: Arial, sans-serif;
  }

  input[type='text'] {
    width: 100%;
    padding: 10px;
    font-size: 16px;
    border-radius: var(--small-radius);
    background: #313131;
    border: 0;
    /* border-left: 3px solid
      var(--primary-dark, color-mix(in srgb, var(--primary), #000 15%)); */
    margin-top: 1rem;
    margin-bottom: 0.5rem;
    color: var(--main-text);
    grid-column: 1 / -1;
    transition: border-color 0.2s ease;

    &:focus {
      outline: none;
    }
  }

  .dropdown-menu {
    position: absolute;
    top: 100%;
    left: 0;
    width: 100%;
    max-height: 200px;
    overflow-y: auto;
    border-radius: var(--small-radius);
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
    background: var(--secondary);
    color: white;
  }

  .dropdown-item img {
    width: 24px;
    height: 24px;
    margin-right: 10px;
  }

  .note {
    font-size: 12px;
    color: var(--secondary-text);
    margin: 0;
  }
  .info-container {
    display: flex;
    padding: 10px;
    align-items: flex-start;
  }
  .icon-info {
    font-size: 14px;
    margin-right: 5px;
    color: var(--secondary-text);
  }
</style>
