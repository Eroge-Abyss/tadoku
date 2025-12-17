<script>
  /** @typedef {import('$lib/types').VndbResult} VndbResult */

  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import Dialog from '$lib/components/Dialog.svelte';
  import { platform } from '@tauri-apps/plugin-os';
  import { appState } from '$lib/state.svelte';
  import { debounce } from '$lib/util';
  import Checkbox from '$lib/components/Checkbox.svelte';
  import InfoNote from '../InfoNote.svelte';
  import SidebarButton from '$lib/components/SidebarButton.svelte';

  const NSFW_RATE = 0.5;

  let showModal = $state(false);
  let search = $state();
  let exe_path = $state();

  /**
   * @type {VndbResult[]}
   */
  let results = $state.raw([]);
  let selectedVn = $state.raw();
  let showImage = $state(false);
  let charactersDownload = $state(false);
  let loading = $state(false);

  // @ts-ignore
  async function updateSearch() {
    const data = await invoke('fetch_vn_info', { key: search });
    results = search ? data : [];
  }

  const openModal = () => (showModal = true);
  const closeModal = () => {
    showModal = false;
    results = [];
    search = '';
    selectedVn = '';
  };

  const pickFile = async () => {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: 'Game exe or shortcut path',
          extensions: ['exe', 'lnk', 'bat', 'sh'],
        },
      ],
    });
    exe_path = file;

    if (!search && !results.length && !selectedVn) {
      // Split by both Windows and Unix path separators
      const pathParts = file?.split(/[\\\/]/) || [];
      if (pathParts.length >= 2) {
        const fileName = pathParts[pathParts.length - 1];
        const parentFolder = pathParts[pathParts.length - 2];
        // Remove extension if it exists
        const fileNameWithoutExt = fileName.includes('.')
          ? fileName.split('.').slice(0, -1).join('.')
          : fileName;
        search = `${parentFolder} ${fileNameWithoutExt}`;
      } else if (pathParts.length === 1) {
        const fileName = pathParts[0];
        search = fileName.includes('.')
          ? fileName.split('.').slice(0, -1).join('.')
          : fileName;
      }
      updateSearch();
    }
  };

  // @ts-ignore
  const selectGame = (game) => {
    selectedVn = game;
    showImage = false;
    results = [];
    search = '';
  };

  /**
   * @param {VndbResult} vn
   */
  const saveGame = async (vn) => {
    if (!vn || vn.id === undefined) {
      alert('Please select a game from the list.');
      return;
    }

    if (!exe_path) {
      alert('Please select a game executable file.');
      return;
    }

    loading = true;
    try {
      const gameData = {
        title: vn.title,
        alt_title: vn.alttitle,
        description: vn.description || 'No Description',
        exe_file_path: exe_path,
        process_file_path: exe_path,
        categories: [],
        icon_url: null,
        image_url: vn.image.url,
        is_pinned: false,
        is_nsfw: vn.image.sexual > NSFW_RATE,
        playtime: 0,
        today_playtime: 0,
        last_play_date: null,
        characters: [],
        last_played: null,
        first_played: null,
        notes: '',
      };

      await appState.saveGame(vn.id, gameData, {
        include_characters: charactersDownload,
      });

      closeModal();
    } catch (error) {
      console.error('Error saving game:', error);
      // @ts-ignore
      alert(`Failed to save game: ${error.message || 'Unknown error'}`);
    } finally {
      loading = false;
    }
  };
</script>

<section>
  <SidebarButton onclick={openModal} tooltip="Add Game">
    <span class="add-icon">+</span>
  </SidebarButton>

  <Dialog show={showModal} close={closeModal}>
    {#snippet header()}
      Add a game
    {/snippet}

    <section class="game-form">
      <div class="search-dropdown">
        <div class="search-input-wrapper">
          <i class="fa-solid fa-magnifying-glass search-icon"></i>
          <input
            type="text"
            bind:value={search}
            autocomplete="one-time-code"
            onkeyup={debounce(updateSearch)}
            placeholder="Search by name or ID..."
          />
        </div>

        {#if results.length > 0}
          <div id="suggestions">
            {#each results as vn}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div class="suggestion-item" onclick={() => selectGame(vn)}>
                <div class="suggestion-image">
                  {#if vn?.image?.sexual < NSFW_RATE}
                    <img src={vn?.image?.url} alt={vn?.title} />
                  {:else}
                    <img src={vn?.image?.url} alt={vn?.title} class="blur" />
                  {/if}
                </div>
                <div class="suggestion-content">
                  <p class="suggestion-title">{vn?.title}</p>
                  <p class="suggestion-id">{vn?.id}</p>
                </div>
              </div>
            {/each}
          </div>
        {:else if search && results.length === 0}
          <div class="empty-state">
            <i class="fa-solid fa-circle-xmark"></i>
            <p>No games found</p>
            <span>Try a different search term or ID</span>
          </div>
        {/if}
      </div>

      {#if selectedVn}
        <div class="selected-suggestion">
          <div class="selected-image">
            {#if selectedVn.image.sexual < NSFW_RATE || showImage}
              <img src={selectedVn.image.url} alt={selectedVn.title} />
            {:else}
              <img
                src={selectedVn.image.url}
                alt={selectedVn.title}
                class="blur"
              />
            {/if}
          </div>
          <div class="selected-content">
            <p class="selected-suggestion-title">
              {selectedVn.title}
            </p>
            <p class="selected-suggestion-id">
              {selectedVn.id}
            </p>
          </div>
        </div>
      {/if}

      <div class="form-group characters">
        <Checkbox
          id="characters"
          label="Include Characters"
          bind:checked={charactersDownload}
        />
      </div>

      {#if platform() === 'linux'}
        <InfoNote>
          If running via a script (e.g., Lutris), add the script as the
          executable and the original EXE as a process path in game settings if
          not detected.
        </InfoNote>
      {/if}

      <InfoNote>
        If you're using a launcher for this novel, please add its process from
        the game details page.
      </InfoNote>

      <button onclick={pickFile}>Select Game Executable</button>
      <button
        disabled={loading}
        class="save-button"
        onclick={() => saveGame(selectedVn)}
      >
        {#if loading}
          Saving...
        {:else}
          Save
        {/if}
      </button>
    </section>
  </Dialog>
</section>

<style>
  .blur {
    filter: blur(5px);
    transition: filter 0.2s ease-in-out;
  }
  .blur:hover {
    filter: blur(0);
  }

  .add-icon {
    font-size: 2.5rem;
    color: var(--primary);
  }

  .game-form {
    margin: 1rem;
  }

  .search-dropdown {
    position: relative;
    margin-bottom: 1rem;
  }

  .search-input-wrapper {
    position: relative;
  }

  .game-form .form-group {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
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

  .search-dropdown input[type='text'] {
    height: 40px;
    width: 100%;
    background-color: var(--accent);
    border: 1px solid transparent;
    border-radius: var(--small-radius);
    padding: 12px 12px 12px 40px;
    color: var(--main-text);
    box-sizing: border-box;
    font-size: 15px;
    transition: all 0.2s ease;
  }

  .search-dropdown input[type='text']::placeholder {
    color: var(--secondary-text);
  }

  .search-dropdown input[type='text']:focus {
    outline: none;
    border-color: var(--primary);
    background: color-mix(in srgb, var(--accent), white 5%);
  }

  .game-form .form-group.characters {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 1rem;
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
  }

  .game-form button:hover {
    background-color: color-mix(in srgb, var(--accent), white 10%);
  }

  #suggestions {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    width: 100%;
    max-height: 280px;
    overflow-y: auto;
    overflow-x: hidden;
    border-radius: var(--small-radius);
    background: var(--accent);
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow:
      0 8px 24px rgba(0, 0, 0, 0.4),
      0 2px 8px rgba(0, 0, 0, 0.2);
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

  /* Custom scrollbar */
  #suggestions::-webkit-scrollbar {
    width: 8px;
  }

  #suggestions::-webkit-scrollbar-track {
    background: var(--accent);
    border-radius: var(--small-radius);
  }

  #suggestions::-webkit-scrollbar-thumb {
    background: color-mix(in srgb, var(--accent), white 20%);
    border-radius: 4px;
  }

  #suggestions::-webkit-scrollbar-thumb:hover {
    background: color-mix(in srgb, var(--accent), white 30%);
  }

  /* Suggestion Item Styling */
  .suggestion-item {
    display: flex;
    align-items: center;
    padding: 12px;
    cursor: pointer;
    transition: all 0.15s ease;
    color: var(--main-text);
    gap: 12px;
  }

  .suggestion-item:hover {
    background: var(--secondary);
    padding-left: 16px;
  }

  .suggestion-item:not(:last-child) {
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  /* Suggestion Text Styling */
  .suggestion-content {
    flex: 1;
    min-width: 0;
  }

  .suggestion-title {
    font-size: 14px;
    font-weight: 500;
    color: var(--main-text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-bottom: 2px;
  }

  .suggestion-id {
    font-size: 12px;
    color: var(--secondary-text);
  }

  /* Suggestion Image Styling */
  .suggestion-image {
    flex-shrink: 0;
  }

  .suggestion-image img {
    width: 60px;
    height: 60px;
    border-radius: var(--small-radius);
    object-fit: cover;
  }

  .selected-suggestion img {
    width: 100px;
    height: 100px;
    border-radius: var(--small-radius);
    object-fit: cover;
  }

  .selected-suggestion {
    margin-bottom: 1rem;
    padding: 12px;
    background: var(--accent);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: var(--small-radius);
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .selected-image {
    flex-shrink: 0;
  }

  .selected-suggestion img {
    width: 80px;
    height: 80px;
    border-radius: var(--small-radius);
    object-fit: cover;
  }

  .selected-content {
    flex: 1;
    min-width: 0;
  }

  .selected-suggestion-title {
    font-size: 15px;
    font-weight: 500;
    color: var(--main-text);
    margin-bottom: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .selected-suggestion-id {
    color: var(--secondary-text);
    font-size: 13px;
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

  .empty-state {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px 16px;
    color: var(--secondary-text);
    text-align: center;
    background: var(--accent);
    border-radius: var(--small-radius);
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow:
      0 8px 24px rgba(0, 0, 0, 0.4),
      0 2px 8px rgba(0, 0, 0, 0.2);
    animation: slideDown 0.2s ease;
    z-index: 1000;
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

  /*
    The following styles were for the old custom checkbox and are no longer needed
    since we are using the Checkbox.svelte component.
  */
  /*
  .custom-checkbox {
    position: relative;
    display: inline-block;
    width: 16px;
    height: 16px;
  }

  .custom-checkbox input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .checkmark {
    position: absolute;
    top: 0;
    left: 0;
    height: 16px;
    width: 16px;
    background-color: #313131;
    border: 2px solid #5d5d5d;
    border-radius: var(--small-radius);
    cursor: pointer;
  }

  .custom-checkbox input:checked ~ .checkmark {
    background-color: var(--primary);
    border-color: #5d5d5d;
  }

  .checkmark:after {
    content: '';
    position: absolute;
    display: none;
  }

  .custom-checkbox input:checked ~ .checkmark:after {
    display: block;
  }

  .custom-checkbox .checkmark:after {
    left: 4px;
    top: 1px;
    width: 4px;
    height: 8px;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
  }
  */
</style>
