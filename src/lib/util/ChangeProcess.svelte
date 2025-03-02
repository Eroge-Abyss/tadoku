<script>
  import CloseIcon from '$lib/util/CloseIcon.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let {
    isOpen = $bindable(),
    gameId,
    processList,
    selected,
    // onConfirm,
    title = 'Change game process path',
    message = 'Are you sure you want to proceed?',
  } = $props();

  $inspect(processList);

  let exe_path = $state();
  let loading = $state(false);

  async function onConfirm(selectedProcessPath) {
    await invoke('update_process', {
      gameId,
      newProcessPath: selectedProcessPath,
    });
  }

  // Close the modal
  function closeModal() {
    isOpen = false;
  }

  // Handle the "OK" button click
  function handleConfirm() {
    if (onConfirm) {
      onConfirm(exe_path); // Run the provided function
    }
    closeModal(); // Close the modal
  }

  import { appState } from '../../routes/state.svelte';

  let searchTerm = $state(''); // Reactive search term

  // onMount(async () => {
  //   appState.loadGames();
  // });
  // Filtered items based on search term
  $inspect(processList);
  let filteredItems = $derived(
    processList.filter((item) =>
      item.title.toLowerCase().includes(searchTerm.toLowerCase()),
    ),
  );

  const selectItem = (item) => {
    selected = item;
    searchTerm = item;
    isOpen = false;
  };
</script>

<section>
  <!--section class="modal process-selector" class:open={showProcessSelector}>
      <div class="process-selector-content">
         <span onclick={closeProcessSelector}>
          <CloseIcon style="font-size: 24px;" />
        </span>
      // we will probably use this later
    </div>
  </section-->
  <section class="modal" class:open={isOpen}>
    <section class="modal__content">
      <header>
        <h3>Add a game</h3>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
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
            onfocus={() => (isOpen = true)}
          />

          {#if isOpen}
            <div class="dropdown-menu show">
              {#each filteredItems as item}
                <div
                  onclick={() => selectItem(item.exe_path)}
                  class="dropdown-item"
                >
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
        <button disabled={loading} class="save-button" onclick={handleConfirm}>
          {#if loading}
            loading...
          {:else}
            Save
          {/if}
        </button>
        <button onclick={closeModal} style="background: #f7768e">Cancel</button>
      </section>
    </section>
  </section>
</section>

<!-- Hide dropdown when clicking outside -->
<svelte:window
  on:click={(e) => {
    if (!e.target?.closest('.dropdown')) {
      isOpen = false;
    }
  }}
/>

<style>
  .blur {
    filter: blur(5px);
    transition: filter 0.2s ease-in-out;
  }
  .blur:hover {
    filter: blur(0);
  }
  #btn__add {
    border: 0;
    background: #313131;
    color: #5d5d5d;
    border: 2px solid #5d5d5d;
    height: 56px;
    width: 56px;
    font-size: 2.5rem;
    text-align: center;
    margin: auto;
    border-radius: 5px;
    cursor: pointer;
  }

  .modal {
    position: fixed;
    height: 100%;
    width: 100%;
    z-index: 2;
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
    &.process-selector {
      z-index: 3;
    }
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
        & .form-group {
          display: grid;
          grid-template-columns: repeat(2, 1fr);
          gap: 1rem;
          & input {
            background-color: #313131;
            border: 0;
            padding: 0.5rem;
            color: var(--main-text);
            max-width: 400px;
          }

          &.characters {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            margin-top: 1rem;
            & > * {
              cursor: pointer;
            }
          }
        }

        & button {
          border: 0;
          background-color: #313131;
          color: #fff;
          width: 100%;
          padding: 0.5rem;
          font-size: 18px;
          margin-top: 1rem;
          cursor: pointer;
        }
      }
    }
  }

  #suggestions {
    margin-top: 10px;
    max-width: 400px;
    max-height: 200px;
    overflow-y: scroll;
    overflow-x: hidden;
  }

  /* Suggestion Item Styling */
  .suggestion-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    background-color: #1b1b1b;
    border-radius: 4px;
    margin-bottom: 5px;
    cursor: pointer;
    transition: background-color 0.3s ease;
    color: var(--main-text);
  }

  .suggestion-item:hover {
    background-color: #bb9af7;

    & .suggestion-id {
      color: #cfc9c2;
    }
  }

  /* Suggestion Text Styling */
  .suggestion-text {
    flex: 1;
    padding: 1rem;
  }

  .suggestion-title {
    font-size: 16px;
  }

  .suggestion-id {
    font-size: 14px;
    color: #aaa;
  }

  /* Suggestion Image Styling */
  .suggestion-image img,
  .selected-suggestion img {
    width: 60px;
    height: 60px;
    border-radius: 4px;
    object-fit: cover;
  }

  .selected-suggestion {
    margin-top: 20px;
    padding: 10px;
    background-color: #1b1b1b;
    border-radius: 4px;
    color: #fff;
    display: flex;
    align-items: center;
  }

  .selected-suggestion img {
    width: 100px;
    height: 100px;
    border-radius: 4px;
    margin-top: 10px;
  }

  .selected-suggestion-title {
    font-size: 16px;
    color: var(--main-text);
  }

  .selected-suggestion-id {
    color: #aaa;
    font-size: 14px;
  }

  .save-button {
    background: #9ece6a !important;
    &[disabled] {
      opacity: 0.5;
    }
  }

  .process-selector {
    & .process-selector-content {
      position: relative;
      height: 100%;
      width: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      & span {
        position: absolute;
        top: 0;
        right: 0;
        margin: 2rem;
        color: var(--secondary-text);
        cursor: pointer;
        transition: color 0.2s ease-in-out;
        &:hover {
          color: var(--main-text);
        }
      }
    }
  }

  .switch-container {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .switch {
    position: relative;
    width: 40px;
    height: 22px;
    background-color: #ccc;
    border-radius: 11px;
    padding: 0;
    border: none;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .switch.active {
    background-color: var(--main-mauve);
  }

  .switch-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 18px;
    height: 18px;
    background-color: white;
    border-radius: 50%;
    transition: transform 0.3s ease;
  }

  .switch.active .switch-thumb {
    transform: translateX(18px);
  }

  .switch-label {
    font-size: 12px;
  }

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
