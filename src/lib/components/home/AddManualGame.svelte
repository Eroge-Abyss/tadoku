<script lang="ts">
  import { type GameDto } from '$lib/types';
  import { toast } from 'svelte-sonner';
  import { platform } from '@tauri-apps/plugin-os';
  import { onMount } from 'svelte';
  import { gamesStore } from '$lib/stores/games.svelte';
  import { pickExecutable, pickImage } from '$lib/util';
  import Checkbox from '$lib/components/Checkbox.svelte';
  import InfoNote from '../InfoNote.svelte';

  let { closeModal }: { closeModal: () => void } = $props();

  let exe_path = $state<string | null>(null);
  let loading = $state(false);
  let currentPlatform = $state('');

  // Manual mode state
  let manualTitle = $state('');
  let manualAltTitle = $state('');
  let manualDescription = $state('');
  let manualImagePath = $state('');
  let manualIsNsfw = $state(false);

  onMount(() => {
    currentPlatform = platform();
  });

  const handlePickFile = async () => {
    const file = await pickExecutable();
    if (file) {
      exe_path = file;
    }
  };

  const handlePickImage = async () => {
    const file = await pickImage();
    if (file) {
      manualImagePath = file;
    }
  };

  const saveManualGame = async () => {
    if (!manualTitle.trim()) {
      toast.error('Please enter a game title.');
      return;
    }

    if (!exe_path) {
      toast.error('Please select a game executable file.');
      return;
    }

    loading = true;
    try {
      // Generate a unique local ID with "l" prefix using a random UUID.
      const gameId = `l${crypto.randomUUID().replace(/-/g, '').slice(0, 12)}`;

      const gameData: GameDto = {
        title: manualTitle.trim(),
        alt_title: manualAltTitle.trim() || null,
        description: manualDescription.trim(),
        exe_file_path: exe_path,
        process_file_path: exe_path,
        characters: null,
        // Pass the raw local path; backend will copy it and store the filename.
        image_url: manualImagePath,
        is_nsfw: manualIsNsfw,
      };

      await gamesStore.saveGame(gameId, gameData, {
        include_characters: false,
      });

      toast.success('Game saved successfully!');
      closeModal();
    } catch (error) {
      console.error('Error saving manual game:', error);
    } finally {
      loading = false;
    }
  };
</script>

<div class="manual-form">
  <label class="field-label" for="manual-title">
    Title <span class="required">*</span>
  </label>
  <input
    id="manual-title"
    type="text"
    bind:value={manualTitle}
    placeholder="Game title"
    class="text-input"
  />

  <label class="field-label" for="manual-alt-title">
    Alt Title <span class="optional">(optional)</span>
  </label>
  <input
    id="manual-alt-title"
    type="text"
    bind:value={manualAltTitle}
    placeholder="Original / alternative title"
    class="text-input"
  />

  <label class="field-label" for="manual-description">
    Description <span class="optional">(optional)</span>
  </label>
  <textarea
    id="manual-description"
    bind:value={manualDescription}
    placeholder="Short description…"
    class="text-input textarea"
    rows="3"
  ></textarea>

  <div class="form-group characters">
    <Checkbox
      id="manual-nsfw"
      label="Mark as NSFW"
      bind:checked={manualIsNsfw}
    />
  </div>

  <button onclick={handlePickImage} class="pick-image-btn">
    <i class="fa-solid fa-image"></i>
    {manualImagePath ? 'Change Cover Image' : 'Select Cover Image (optional)'}
  </button>

  {#if manualImagePath}
    <p class="image-path-hint">{manualImagePath}</p>
  {/if}

  {#if currentPlatform === 'linux'}
    <InfoNote>
      If running via a script (e.g., Lutris), add the script as the executable
      and the original EXE as a process path in game settings if not detected.
    </InfoNote>
  {/if}

  <InfoNote>
    If you're using a launcher for this novel, please add its process from the
    game details page.
  </InfoNote>

  <div class="form-handler">
    <button onclick={handlePickFile}>Select Game Executable</button>
    <button disabled={loading} class="save-button" onclick={saveManualGame}>
      {#if loading}
        Saving...
      {:else}
        Save
      {/if}
    </button>
  </div>
</div>

<style>
  .manual-form {
    display: flex;
    flex-direction: column;
  }

  .field-label {
    font-size: 13px;
    color: var(--secondary-text);
    margin-bottom: 4px;
    margin-top: 10px;
  }

  .field-label .required {
    color: var(--primary);
  }

  .field-label .optional {
    opacity: 0.6;
  }

  .text-input {
    width: 100%;
    background-color: var(--accent);
    border: 1px solid transparent;
    border-radius: var(--small-radius);
    padding: 8px 12px;
    color: var(--main-text);
    box-sizing: border-box;
    font-size: 14px;
    transition: border-color 0.2s ease;
    font-family: inherit;
  }

  .text-input:focus {
    outline: none;
    border-color: var(--primary);
    background: color-mix(in srgb, var(--accent), white 5%);
  }

  .text-input.textarea {
    resize: vertical;
    min-height: 64px;
  }

  .pick-image-btn {
    margin-top: 10px;
    display: flex;
    align-items: center;
    gap: 6px;
    justify-content: center;
  }

  .image-path-hint {
    font-size: 11px;
    color: var(--secondary-text);
    margin: 4px 0 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
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
