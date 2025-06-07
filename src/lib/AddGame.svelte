<script>
  /** @typedef {import('$lib/types').VndbResult} VndbResult */

  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { appState } from '../routes/state.svelte';
  import Dialog from '$lib/util/Dialog.svelte';

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
  async function updateSearch(e) {
    search = e.target.value;
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

  // @ts-ignore
  const debounce = (v) => {
    let timer;
    clearTimeout(timer);
    timer = setTimeout(() => {
      updateSearch(v); // TODO: make this function generic
    }, 750);
  };

  const pickFile = async () => {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: 'Game exe or shortcut path',
          extensions: ['exe', 'lnk', 'bat'],
        },
      ],
    });
    exe_path = file;
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
        description: vn.description || 'No Description',
        exe_file_path: exe_path,
        process_file_path: exe_path,
        categories: [],
        icon_url: null,
        image_url: vn.image.url,
        is_pinned: false,
        is_nsfw: vn.image.sexual > NSFW_RATE,
        playtime: 0,
        characters: [],
        last_played: null,
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
  <button id="btn__add" onclick={openModal}> + </button>

  <Dialog show={showModal} close={closeModal}>
    {#snippet header()}
      Add a game
    {/snippet}

    <section class="game-form">
      <div class="form-group">
        <!-- No questions asked (about autocomplete). it just works -->
        <input
          type="text"
          value={search}
          autocomplete="one-time-code"
          onkeyup={(e) => debounce(e)}
          placeholder="Name or ID"
        />
      </div>
      <div class="form-group characters">
        <label for="characters" class="custom-checkbox">
          <input
            type="checkbox"
            id="characters"
            bind:checked={charactersDownload}
          />
          <span class="checkmark"></span>
        </label>
        <label for="characters">Include Characters</label>
      </div>
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
            <div class="suggestion-text">
              <p class="suggestion-title">{vn?.title}</p>
              <p class="suggestion-id">{vn?.id}</p>
            </div>
          </div>
        {/each}
      </div>

      {#if selectedVn}
        <div class="selected-suggestion">
          {#if selectedVn.image.sexual < NSFW_RATE || showImage}
            <img src={selectedVn.image.url} alt={selectedVn.title} />
          {:else}
            <img
              src={selectedVn.image.url}
              alt={selectedVn.title}
              class="blur"
            />
          {/if}
          <div class="suggestion-text">
            <p class="selected-suggestion-title">
              {selectedVn.title}
            </p>
            <p class="selected-suggestion-id">
              {selectedVn.id}
            </p>
          </div>
        </div>
      {/if}

      <div class="info-container">
        <span class="icon-info">
          <i class="fa-solid fa-info-circle"></i>
        </span>
        <p class="note">
          If you're using a launcher for this novel, please add its process from
          the game details page.
        </p>
      </div>

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
  .blur {
    filter: blur(5px);
    transition: filter 0.2s ease-in-out;
  }
  .blur:hover {
    filter: blur(0);
  }
  #btn__add {
    border: 0;
    width: 56px;
    height: 56px;
    font-size: 2.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: auto;
    cursor: pointer;
    color: var(--primary);
    background: rgba(185, 154, 250, 0.17);
    border-radius: var(--big-radius);
    box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(185, 154, 250, 0.13);
    transition: all 0.3s ease;
  }

  #btn__add:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 12px rgba(0, 0, 0, 0.25);
    background: rgba(185, 154, 250, 0.25);
  }

  #btn__add:active {
    transform: translateY(0);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .game-form {
    margin: 1rem;
  }
  .game-form .form-group {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }
  .game-form .form-group input[type='text'] {
    height: 40px;
    width: 100%;
    background-color: #313131;
    border: 0;
    border-radius: var(--small-radius);
    padding: 0.5rem;
    color: var(--main-text);
    box-sizing: border-box;
    grid-column: 1 / -1;
    transition: border-color 0.2s ease;
  }

  .game-form .form-group input[type='text']:focus {
    outline: none;
  }

  .game-form .form-group.characters {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 1rem;
  }
  .game-form .form-group.characters > * {
    cursor: pointer;
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
  }

  .game-form button:hover {
    background-color: #404040;
  }

  #suggestions {
    margin-top: 10px;
    /* max-width: 400px; */
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
    background-color: color-mix(in srgb, black 30%, var(--main-background) 80%);
    border-radius: var(--small-radius);
    margin-bottom: 5px;
    cursor: pointer;
    transition: background-color 0.3s ease;
    color: var(--main-text);
  }

  .suggestion-item:hover {
    background-color: color-mix(
      in srgb,
      var(--primary) 2.5%,
      var(--secondary) 92%
    );

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
    border-radius: var(--small-radius);
    object-fit: cover;
  }

  .selected-suggestion {
    margin-top: 20px;
    padding: 10px;
    background-color: color-mix(in srgb, black 30%, var(--main-background) 80%);
    border-radius: var(--small-radius);
    color: #fff;
    display: flex;
    align-items: center;
  }

  .selected-suggestion img {
    width: 100px;
    height: 100px;
    border-radius: var(--small-radius);
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

  /* Style for the custom checkbox */
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
</style>
