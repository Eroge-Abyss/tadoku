<script lang="ts">
  import type { Process } from '$lib/types';
  import CloseIcon from '$lib/util/CloseIcon.svelte';
  import { invoke } from '@tauri-apps/api/core';
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
  <section class="modal" class:open={isOpen}>
    <section class="modal__content">
      <header>
        <h3>{title}</h3>
        <span onclick={closeModal}>
          <CloseIcon style="font-size: 24px;" />
        </span>
      </header>
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
                  <img
                    src={item.icon}
                    alt={item.title}
                    width={32}
                    height={32}
                  />
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
            If you can't find your game, try setting it to windowed then return
            to full screen when done.
          </p>
        </div>

        <button disabled={loading} class="save-button" onclick={handleConfirm}>
          {#if loading}
            Saving...
          {:else}
            Save
          {/if}
        </button>
        <button onclick={closeModal}>Cancel</button>
      </section>
    </section>
  </section>
</section>

<!-- Hide dropdown when clicking outside -->
<svelte:window
  on:click={(e) => {
    if (!e.target?.closest('.dropdown')) {
      isDropdownOpen = false;
    }
  }}
/>

<style>
  .modal {
    position: fixed;
    height: 100%;
    width: 100%;
    z-index: 3;
    top: 0;
    left: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    color: #fff;
    background: rgba(0, 0, 0, 0.6);
    opacity: 0;
    pointer-events: none;
    transition: all 0.2s ease-in-out;
    /* Start scaled down */
    &.open {
      opacity: 1;
      pointer-events: all;

      & .modal__content {
        transform: translate(0, 0) scale(1); /* Scale up */
      }
    }

    & .modal__content {
      background-color: var(--main-background);
      padding: 1rem;
      width: 500px;
      display: flex;
      flex-direction: column;
      transform: translate(0, 100%) scale(0.8);
      transition: all 0.2s ease-in-out;
      & header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        color: var(--main-text);
        span {
          color: #444;
          cursor: pointer;
          transition: color 0.2s ease-in-out;
          &:hover {
            color: var(--main-text);
          }
        }
      }

      & .game-form {
        margin: 1rem;

        & button {
          border: 0;
          background-color: #313131;
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
      }
    }
  }

  .save-button {
    background: var(--primary) !important;
    &[disabled] {
      opacity: 0.5;
    }
    &:hover:not([disabled]) {
      background: var(--primary-dark, color-mix(in srgb, var(--primary), #000 10%)) !important;
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
    border-radius: 4px;
    background: #313131;
    border: 0;
    border-left: 3px solid var(--primary-dark, color-mix(in srgb, var(--primary), #000 15%));
    margin-top: 1rem;
    margin-bottom: 0.5rem;
    color: var(--main-text);
    grid-column: 1 / -1;
    transition: border-color 0.2s ease;
    
    &:focus {
      outline: none;
      border-left: 3px solid var(--primary);
    }
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
