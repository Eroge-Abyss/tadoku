<script lang="ts">
  import { type GameDto } from '$lib/types';
  import { toast } from 'svelte-sonner';
  import { platform } from '@tauri-apps/plugin-os';
  import { onMount } from 'svelte';
  import { gamesStore } from '$lib/stores/games.svelte';
  import { pickExecutable, debounce } from '$lib/util';
  import Checkbox from '$lib/components/Checkbox.svelte';
  import InfoNote from '../InfoNote.svelte';
  import { useVndbSearch } from '$lib/composables/useVndbSearch.svelte';

  let { closeModal }: { closeModal: () => void } = $props();

  const NSFW_RATE = 0.5;

  let exe_path = $state<string | null>(null);
  let loading = $state(false);
  let currentPlatform = $state('');
  let showImage = $state(false);
  let charactersDownload = $state(false);

  const vndb = useVndbSearch();

  onMount(() => {
    currentPlatform = platform();
  });

  const handlePickFile = async () => {
    const file = await pickExecutable();
    if (file) {
      exe_path = file;
      vndb.searchFromPath(file);
    }
  };

  const saveVndbGame = async () => {
    if (!vndb.selectedVn || vndb.selectedVn.id === undefined) {
      toast.error('Please select a game from the list.');
      return;
    }

    if (!exe_path) {
      toast.error('Please select a game executable file.');
      return;
    }

    loading = true;
    try {
      const gameData: GameDto = {
        title: vndb.selectedVn.title,
        alt_title: vndb.selectedVn.alttitle,
        description: vndb.selectedVn.description || 'No Description',
        exe_file_path: exe_path,
        process_file_path: exe_path,
        image_url: vndb.selectedVn.image.url,
        is_nsfw: vndb.selectedVn.image.sexual > NSFW_RATE,
        characters: [],
      };

      await gamesStore.saveGame(vndb.selectedVn.id, gameData, {
        include_characters: charactersDownload,
      });

      toast.success('Game saved successfully!');
      closeModal();
    } catch (error) {
      console.error('Error saving game:', error);
    } finally {
      loading = false;
    }
  };
</script>

<div class="search-dropdown">
  <div class="search-input-wrapper">
    <i class="fa-solid fa-magnifying-glass search-icon"></i>
    <input
      type="text"
      bind:value={vndb.search}
      autocomplete="one-time-code"
      onkeyup={debounce(vndb.updateSearch)}
      placeholder="Search by name or ID..."
    />
  </div>

  {#if vndb.results.length > 0}
    <div id="suggestions" role="listbox">
      {#each vndb.results as vn (vn.id)}
        <button
          role="option"
          aria-selected={vndb.selectedVn === vn}
          class="suggestion-item"
          onclick={() => vndb.selectGame(vn)}
        >
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
        </button>
      {/each}
    </div>
  {:else if vndb.search && vndb.results.length === 0}
    <div class="empty-state">
      <i class="fa-solid fa-circle-xmark"></i>
      <p>No games found</p>
      <span>Try a different search term or ID</span>
    </div>
  {/if}
</div>

{#if vndb.selectedVn}
  <div class="selected-suggestion">
    <div class="selected-image">
      {#if vndb.selectedVn.image.sexual < NSFW_RATE || showImage}
        <img src={vndb.selectedVn.image.url} alt={vndb.selectedVn.title} />
      {:else}
        <img
          src={vndb.selectedVn.image.url}
          alt={vndb.selectedVn.title}
          class="blur"
        />
      {/if}
    </div>
    <div class="selected-content">
      <p class="selected-suggestion-title">
        {vndb.selectedVn.title}
      </p>
      <p class="selected-suggestion-id">
        {vndb.selectedVn.id}
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

{#if currentPlatform === 'linux'}
  <InfoNote>
    If running via a script (e.g., Lutris), add the script as the executable and
    the original EXE as a process path in game settings if not detected.
  </InfoNote>
{/if}

<InfoNote>
  If you're using a launcher for this novel, please add its process from the
  game details page.
</InfoNote>

<div class="form-handler">
  <button onclick={handlePickFile}>Select Game Executable</button>
  <button disabled={loading} class="save-button" onclick={saveVndbGame}>
    {#if loading}
      Saving...
    {:else}
      Save
    {/if}
  </button>
</div>

<style>
  .blur {
    filter: blur(5px);
    transition: filter 0.2s ease-in-out;
  }
  .blur:hover {
    filter: blur(0);
  }

  .search-dropdown {
    position: relative;
    margin-bottom: 1rem;
  }

  .search-input-wrapper {
    position: relative;
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
  button.suggestion-item {
    background: transparent;
    border: none;
    margin: 0;
    font: inherit;
    text-align: left;
    box-sizing: border-box;
    width: 100%;
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
    text-align: left;
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

  .form-group.characters {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .form-handler {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1rem;
  }

  button {
    border: 0;
    background-color: var(--accent);
    border-radius: var(--small-radius);
    color: var(--main-text);
    width: 100%;
    padding: 0.5rem;
    font-size: 18px;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  button:hover {
    background-color: color-mix(in srgb, var(--accent), white 10%);
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
</style>
