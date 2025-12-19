<script lang="ts">
  import type { Process } from '$lib/types';
  import Dialog from '$lib/components/Dialog.svelte';
  import { appState } from '$lib/state.svelte';
  import InfoNote from '../InfoNote.svelte';

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
        <div class="search-input-wrapper">
          <i class="fa-solid fa-magnifying-glass search-icon"></i>
          <input
            type="text"
            placeholder="Search processes..."
            bind:value={searchTerm}
            onfocus={() => (isDropdownOpen = true)}
          />
        </div>

        {#if isDropdownOpen}
          <div class="dropdown-menu show">
            {#if filteredItems.length === 0}
              <div class="empty-state">
                <i class="fa-solid fa-circle-xmark"></i>
                <p>No processes found</p>
                <span>Try a different search term</span>
              </div>
            {:else}
              {#each filteredItems as item}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div onclick={() => selectItem(item)} class="dropdown-item">
                  <div class="item-icon">
                    {#if item.icon}
                      <img
                        src={item.icon}
                        alt={item.title}
                        width={32}
                        height={32}
                      />
                    {:else}
                      <div class="placeholder-icon">
                        <i class="fa-solid fa-circle-xmark"></i>
                      </div>
                    {/if}
                  </div>
                  <div class="item-content">
                    <span class="item-title">{item.title}</span>
                    <span class="item-path">{item.exe_path}</span>
                  </div>
                </div>
              {/each}
            {/if}
          </div>
        {/if}
      </div>

      <InfoNote>
        If you can't find your game, try setting it to windowed then return to
        full screen when done.
      </InfoNote>

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
    background-color: var(--accent);
    border-radius: var(--small-radius);
    color: var(--main-text);
    width: 100%;
    padding: 0.5rem;
    font-size: 18px;
    margin-top: 1rem;
    cursor: pointer;
    transition: background-color 0.3s ease;

    &:hover {
      background-color: color-mix(in srgb, var(--accent), white 10%);
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

  .search-input-wrapper {
    position: relative;
    margin-top: 1rem;
    margin-bottom: 0.5rem;
  }

  .search-icon {
    position: absolute;
    left: 14px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 16px;
    color: var(--secondary-text);
    pointer-events: none;
  }

  input[type='text'] {
    width: 100%;
    padding: 12px 12px 12px 40px;
    font-size: 15px;
    border-radius: var(--small-radius);
    background: var(--accent);
    border: 1px solid transparent;
    color: var(--main-text);
    transition: all 0.2s ease;

    &::placeholder {
      color: var(--secondary-text);
    }

    &:focus {
      outline: none;
      border-color: var(--primary);
      background: color-mix(in srgb, var(--accent), white 5%);
    }
  }

  .dropdown-menu {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    width: 100%;
    max-height: 280px;
    overflow-y: auto;
    border-radius: var(--small-radius);
    background: var(--accent);
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow:
      0 8px 24px rgba(0, 0, 0, 0.4),
      0 2px 8px rgba(0, 0, 0, 0.2);
    display: none;
    animation: slideDown 0.2s ease;
    z-index: 1000;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .dropdown-menu.show {
    display: block;
  }

  /* Custom scrollbar */
  .dropdown-menu::-webkit-scrollbar {
    width: 8px;
  }

  .dropdown-menu::-webkit-scrollbar-track {
    background: var(--accent);
    border-radius: var(--small-radius);
  }

  .dropdown-menu::-webkit-scrollbar-thumb {
    background: color-mix(in srgb, var(--accent), white 20%);
    border-radius: 4px;
  }

  .dropdown-menu::-webkit-scrollbar-thumb:hover {
    background: color-mix(in srgb, var(--accent), white 30%);
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    padding: 12px;
    cursor: pointer;
    color: var(--main-text);
    transition: all 0.15s ease;
    gap: 12px;
  }

  .dropdown-item:hover {
    background: var(--secondary);
    padding-left: 16px;
  }

  .dropdown-item:not(:last-child) {
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .item-icon {
    flex-shrink: 0;
  }

  .dropdown-item img {
    width: 32px;
    height: 32px;
    border-radius: 6px;
  }

  .item-content {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }

  .item-title {
    font-size: 14px;
    font-weight: 500;
    color: var(--main-text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .item-path {
    font-size: 12px;
    color: var(--secondary-text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .placeholder-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: color-mix(in srgb, var(--accent), white 10%);
    border-radius: 6px;
    color: var(--secondary-text);
    font-size: 18px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px 16px;
    color: var(--secondary-text);
    text-align: center;
  }

  .empty-state i {
    font-size: 48px;
    margin-bottom: 12px;
    opacity: 0.5;
  }

  .empty-state p {
    margin: 0 0 4px 0;
    font-size: 15px;
    font-weight: 500;
    color: var(--main-text);
    opacity: 0.8;
  }

  .empty-state span {
    font-size: 13px;
    color: var(--secondary-text);
  }
</style>
