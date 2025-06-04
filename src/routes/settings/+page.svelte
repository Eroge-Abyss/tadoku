<script lang="ts">
  import { onMount } from 'svelte';
  import { appState } from '../state.svelte.js';
  import { invoke } from '@tauri-apps/api/core';
  import { getVersion } from '@tauri-apps/api/app';
  import { THEMES, COLOR_SWATCHES } from '../../themeConstants.js';
  import type { Theme } from '$lib/types';
  import type { ColorSwatch } from '$lib/types';

  let appVersion = $state<string>();
  let selectedTheme = $state<string>(appState.themeSettings.theme);
  let customColor = $state<string>(appState.themeSettings.accentColor);
  let useCustomColor = $state<boolean>(appState.themeSettings.useCustomColor);
  let disableDiscordPresence = $state<boolean>(false);
  let themeMap = $state<Map<string, Theme>>(new Map());
  const selectHandlers: Record<string, () => void> = {};

  let colorOptionsVisible = $state<boolean>(false);

  $effect(() => {
    if (THEMES.length > 0) {
      const newMap = new Map<string, Theme>();
      THEMES.forEach((THEMES) => {
        newMap.set(THEMES.id, THEMES);
        selectHandlers[THEMES.id] = () => selectTheme(THEMES.id);
      });
      themeMap = newMap;
    }
  });

  $effect(() => {
    selectedTheme = appState.themeSettings.theme;
    customColor = appState.themeSettings.accentColor;
    useCustomColor = appState.themeSettings.useCustomColor;
    colorOptionsVisible = useCustomColor;
  });

  let previewColor = $derived(
    useCustomColor
      ? customColor
      : themeMap.get(selectedTheme)?.primary || THEMES[0]?.primary || '#1e88e5',
  );

  async function toggleDiscordPresence(): Promise<void> {
    try {
      await invoke('set_nsfw_presence_status');

      disableDiscordPresence = await invoke<boolean>(
        'get_nsfw_presence_status',
      );
    } catch (error) {
      console.error('Error toggling Discord presence status:', error);
    }
  }

  onMount(async () => {
    try {
      disableDiscordPresence = await invoke<boolean>(
        'get_nsfw_presence_status',
      );
    } catch (error) {
      console.error('Error fetching Discord presence status:', error);
      disableDiscordPresence = false;
    }

    appVersion = await getVersion();
  });

  function selectTheme(themeId: string): void {
    requestAnimationFrame(() => {
      selectedTheme = themeId;
      appState.updateThemeSettings({ theme: themeId });
    });
  }

  function toggleCustomColor(): void {
    if (useCustomColor) {
      colorOptionsVisible = false;
      setTimeout(() => {
        useCustomColor = false;
        appState.updateThemeSettings({ useCustomColor: false });
      }, 300);
    } else {
      useCustomColor = true;
      appState.updateThemeSettings({ useCustomColor: true });
      setTimeout(() => {
        colorOptionsVisible = true;
      }, 50);
    }
  }

  function selectColor(color: string): void {
    customColor = color;
    useCustomColor = true;
    colorOptionsVisible = true;
    appState.updateThemeSettings({
      accentColor: color,
      useCustomColor: true,
    });
  }

  function handleColorInput(event: Event): void {
    const input = event.target as HTMLInputElement;
    customColor = input.value;
    useCustomColor = true;
    colorOptionsVisible = true;
    appState.updateThemeSettings({
      accentColor: customColor,
      useCustomColor: true,
    });
  }

  async function resetSettings(): Promise<void> {
    appState.resetThemeSettings();

    selectedTheme = appState.themeSettings.theme;
    customColor = appState.themeSettings.accentColor;
    useCustomColor = appState.themeSettings.useCustomColor;
    colorOptionsVisible = useCustomColor;

    try {
      let currentStatus = await invoke<boolean>('get_nsfw_presence_status');

      if (currentStatus !== false) {
        await invoke('set_nsfw_presence_status');
        disableDiscordPresence = await invoke<boolean>(
          'get_nsfw_presence_status',
        );
      } else {
        disableDiscordPresence = false;
      }
    } catch (error) {
      console.error('Error resetting Discord presence status:', error);
    }
  }

  function handleThemeSelection(e: MouseEvent): void {
    const themeItem = (e.target as HTMLElement).closest('.theme-item');
    if (themeItem) {
      const themeId = themeItem.getAttribute('data-theme-id');
      if (themeId) {
        selectTheme(themeId);
      }
    }
  }

  let indexedColorSwatches = $derived<ColorSwatch[]>(
    COLOR_SWATCHES.map((color, index) => ({ color, index })),
  );
</script>

<div class="container">
  <div class="content">
    <div class="header">
      <h1>Settings</h1>
      <p>Customize your Tadoku experience</p>
    </div>

    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="settings-section">
      <h2>Theme Selection</h2>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div class="theme-grid" onclick={handleThemeSelection}>
        {#each THEMES as theme}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="theme-item"
            class:active={selectedTheme === theme.id}
            style="--preview-primary: {theme.primary}; --preview-bg: {theme.background}; --preview-accent: {theme.accent};"
            data-theme-id={theme.id}
          >
            <div class="theme-preview">
              <div class="preview-sidebar"></div>
              <div class="preview-content">
                <div class="preview-card"></div>
                <div class="preview-card"></div>
              </div>
            </div>
            <div class="theme-name">{theme.name}</div>
          </div>
        {/each}
      </div>
    </div>

    <div class="settings-section">
      <h2>Custom Accent Color</h2>
      <div class="custom-color-section">
        <div class="switch-container">
          <!-- svelte-ignore a11y_consider_explicit_label -->
          <button
            class="switch"
            class:active={useCustomColor}
            onclick={toggleCustomColor}
          >
            <span class="switch-thumb"></span>
          </button>
          <span class="switch-label">Use custom accent color</span>
        </div>

        <div class="color-options" class:visible={colorOptionsVisible}>
          <div class="color-picker">
            <input
              type="color"
              bind:value={customColor}
              oninput={handleColorInput}
              disabled={!useCustomColor}
            />
            <span>{customColor}</span>
          </div>

          <div class="color-swatches">
            {#each indexedColorSwatches as { color, index }}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                class="color-swatch"
                style="background-color: {color}; --index: {index};"
                class:active={customColor === color}
                onclick={() => selectColor(color)}
              ></div>
            {/each}
          </div>
        </div>

        <div class="preview" class:visible={colorOptionsVisible}>
          <div class="preview-title">Preview</div>
          <div class="preview-button" style="background-color: {previewColor}">
            Button
          </div>
        </div>
      </div>
    </div>

    <div class="settings-section">
      <h2>App Settings</h2>
      <div class="switch-container">
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button
          class="switch"
          class:active={disableDiscordPresence}
          onclick={toggleDiscordPresence}
        >
          <span class="switch-thumb"></span>
        </button>
        <span class="switch-label"
          >Disable Discord Presence for NSFW content</span
        >
      </div>

      <div class="switch-container">
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button
          class="switch"
          class:active={appState.showRandomButton}
          onclick={() =>
            appState.setShowRandomButton(!appState.showRandomButton)}
        >
          <span class="switch-thumb"></span>
        </button>
        <span class="switch-label">Show the random game button in home</span>
      </div>
    </div>
    <!--
    <div class="settings-section">
      <h2>App Settings</h2>
      <button class="update-button">Check For Updates</button>
      <button class="">DownloadUpdate</button>
    </div>
    -->

    <div class="settings-section">
      <h2>Reset Settings</h2>
      <button class="reset-button" onclick={resetSettings}>
        Reset to Default Settings
      </button>
    </div>
    <div class="version-info">
      Tadoku v{appVersion}
    </div>
  </div>
</div>

<style>
  .version-info {
    display: flex;
    flex-direction: row-reverse;
    user-select: none;
    opacity: 0.5;
    font-family: monospace;
  }
  .container {
    padding: 25px;
    max-width: 1200px;
    margin: auto;
  }

  .content {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 2rem;
    padding: 1rem;
  }

  .header {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .header h1 {
    font-size: 2.5rem;
    font-weight: 600;
    color: var(--main-text);
  }

  .header p {
    color: var(--secondary-text);
    font-size: 1rem;
  }

  .settings-section {
    background-color: var(--accent);
    border-radius: var(--big-radius);
    padding: 1.5rem 2rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    position: relative;
    overflow: hidden;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .settings-section::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 100%;

    pointer-events: none;
  }

  .settings-section h2 {
    font-size: 1.5rem;
    font-weight: 500;
    color: var(--main-text);
    margin-bottom: 0.5rem;
    margin-top: 0.25em;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    padding-bottom: 0.75rem;
  }

  .theme-grid {
    will-change: transform;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 1rem;
    contain: layout style;
  }

  .theme-item {
    background: rgba(49, 49, 49, 0.5);
    border-radius: 8px;
    padding: 1rem;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    border: 1px solid rgba(255, 255, 255, 0.05);
    position: relative;
    overflow: hidden;
    contain: layout paint;
  }

  .theme-item::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 80%;
    height: 100%;
    transition: 0.3s ease-in;
  }

  .theme-item:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.15);
    background: rgba(49, 49, 49, 0.6);
  }

  .theme-item.active {
    border-color: var(--primary);
    box-shadow: 0 0 10px rgba(var(--primary-rgb), 0.2);
  }

  .theme-preview {
    height: 100px;
    display: flex;
    background-color: var(--preview-bg);
    border-radius: var(--small-radius);
    overflow: hidden;
  }

  .preview-sidebar {
    width: 20%;
    height: 100%;
    background-color: #1b1b1b;
  }

  .preview-content {
    width: 80%;
    height: 100%;
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .preview-card {
    height: 35%;
    background-color: var(--preview-accent);
    border-radius: var(--small-radius);
    position: relative;
  }

  .preview-card::before {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    height: 3px;
    width: 70%;
    background-color: var(--preview-primary);
    border-radius: 0 3px 3px 0;
  }

  .theme-name {
    font-size: 14px;
    color: var(--main-text);
    text-align: center;
    margin-top: 0.5rem;
  }
  .custom-color-section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .custom-color-section:has(.color-options.visible) {
    gap: 1rem;
  }

  .custom-color-section:not(:has(.color-options.visible)) {
    /* new fancy selector */
    gap: 0;
  }
  .switch-container {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .switch {
    position: relative;
    width: 40px;
    height: 22px;
    background-color: rgba(68, 68, 68, 0.6);
    border-radius: var(--big-radius);
    padding: 0;
    border: 1px solid rgba(255, 255, 255, 0.1);
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .switch-thumb {
    position: absolute;
    top: 1px;
    will-change: transform;
    left: 1px;
    width: 18px;
    height: 18px;
    background-color: white;
    border-radius: 50%;
    transition: transform 0.3s ease;
  }

  .switch.active {
    background: var(--primary);
  }

  .switch.active .switch-thumb {
    transform: translateX(18px);
  }

  .switch-label {
    font-size: 14px;
    color: var(--main-text);
  }

  .color-options {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    max-height: 0;
    overflow: hidden;
    opacity: 0;
    transform: translateY(-10px);
    transition:
      max-height 0.3s ease,
      opacity 0.3s ease,
      transform 0.3s ease;
    pointer-events: none;
  }

  .color-options.visible {
    max-height: 300px;
    opacity: 1;
    transform: translateY(0);
    pointer-events: all;
  }

  .color-picker {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .color-picker input {
    width: 50px;
    height: 50px;
    border: none;
    cursor: pointer;
    background: none;
  }

  .color-picker span {
    color: var(--main-text);
    font-family: monospace;
    font-size: 15px;
  }

  .color-swatches {
    margin: 5px;
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .color-swatch {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    cursor: pointer;
    transition: transform 0.2s;
    border: 2px solid transparent;
    position: relative;
    overflow: hidden;
  }

  .color-swatch::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.15) 0%,
      transparent 50%
    );
    border-radius: 50%;
  }

  .color-swatch:hover {
    transform: scale(1.1);
  }

  .color-swatch.active {
    border-color: white;
    box-shadow: 0 0 0 2px var(--accent);
  }

  .preview {
    margin-top: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-height: 0;
    overflow: hidden;
    opacity: 0;
    transform: translateY(-10px);
    transition:
      max-height 0.3s ease,
      opacity 0.3s ease,
      transform 0.3s ease,
      margin-top 0.3s ease;
    pointer-events: none;
  }

  .preview.visible {
    margin-top: 1rem;
    max-height: 200px;
    opacity: 1;
    transform: translateY(0);
    pointer-events: all;
  }

  .preview-title {
    font-size: 14px;
    color: var(--secondary-text);
  }

  .preview-button {
    padding: 0.5rem 1rem;
    border-radius: var(--small-radius);
    color: white;
    font-weight: 500;
    text-align: center;
    width: fit-content;
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
  }

  .reset-button {
    background-color: #ef4444;
    color: white;
    border: none;
    padding: 0.75rem 1rem;
    border-radius: var(--small-radius);
    font-weight: 500;
    cursor: pointer;
    width: fit-content;
    transition: background-color 0.2s;
  }

  .reset-button:hover {
    background-color: #dc2626;
  }
</style>
