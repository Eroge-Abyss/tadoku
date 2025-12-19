<script lang="ts">
  import { onMount } from 'svelte';
  import { appState } from '$lib/state.svelte.js';
  import { getVersion } from '@tauri-apps/api/app';
  import { THEMES, COLOR_SWATCHES } from '$lib/constants';
  import type {
    ColorSwatch,
    PlaytimeMode,
    DiscordPresenceMode,
    Theme,
  } from '$lib/types';
  import { appLogDir } from '@tauri-apps/api/path';
  import { revealItemInDir } from '@tauri-apps/plugin-opener';
  import InfoNote from '$lib/components/InfoNote.svelte';

  const EXSTATIC_GITHUB_URL = 'https://github.com/kofta999/exSTATic';

  let appVersion = $state<string>();
  let selectedTheme = $state<string>(appState.themeSettings.theme);
  let customColor = $state<string>(appState.themeSettings.accentColor);
  let useCustomColor = $state<boolean>(appState.themeSettings.useCustomColor);
  let themeMap = $state<Map<string, Theme>>(new Map());
  const selectHandlers: Record<string, () => void> = {};

  let colorOptionsVisible = $state<boolean>(false);
  let selectedPresenceMode = $state<DiscordPresenceMode>(
    appState.discordPresenceMode,
  );
  let playtimeMode = $state<PlaytimeMode>(appState.playtimeMode);

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
    selectedPresenceMode = appState.discordPresenceMode;
  });

  let previewColor = $derived(
    useCustomColor
      ? customColor
      : themeMap.get(selectedTheme)?.primary || THEMES[0]?.primary || '#1e88e5',
  );

  onMount(async () => {
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
      useCustomColor = false;
      appState.updateThemeSettings({ useCustomColor: false });
      setTimeout(() => {
        colorOptionsVisible = false;
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
    try {
      appState.resetThemeSettings();

      selectedTheme = appState.themeSettings.theme;
      customColor = appState.themeSettings.accentColor;
      useCustomColor = appState.themeSettings.useCustomColor;
      colorOptionsVisible = useCustomColor;

      appState.setShowRandomButton(true);
      appState.setDisablePresenceOnNsfw(true);
      appState.setDiscordPresenceMode('All');
    } catch (error) {
      console.error('Error resetting settings:', error);
    }
  }

  async function openLogsDirectory(): Promise<void> {
    try {
      const logsDir = await appLogDir();
      await revealItemInDir(logsDir);
    } catch (error) {
      console.error('Error opening logs directory:', error);
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
      <div class="header-content">
        <h1>Settings</h1>
        <p>Customize your Tadoku experience</p>
      </div>
    </div>

    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="settings-section">
      <div class="section-header">
        <h2>Theme Selection</h2>
        <p class="section-description">Choose your preferred color theme</p>
      </div>
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
      <div class="section-header">
        <h2>Discord Presence</h2>
        <p class="section-description">
          Configure Discord Rich Presence settings
        </p>
      </div>
      <div class="select-container">
        <label for="presence-mode">Mode:</label>
        <select
          id="presence-mode"
          bind:value={selectedPresenceMode}
          onchange={() => appState.setDiscordPresenceMode(selectedPresenceMode)}
        >
          <option value="All">Show Presence for Everything</option>
          <option value="InGame">Show Presence for Games Only</option>
          <option value="None">Disable All Discord Presence</option>
        </select>
      </div>
      <div class="switch-container">
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button
          class="switch"
          class:active={appState.disablePresenceOnNsfw}
          onclick={() =>
            appState.setDisablePresenceOnNsfw(!appState.disablePresenceOnNsfw)}
        >
          <span class="switch-thumb"></span>
        </button>
        <span class="switch-label">Hide image and VNDB link for NSFW games</span
        >
      </div>
    </div>

    <div class="settings-section">
      <div class="section-header">
        <h2>App Settings</h2>
        <p class="section-description">General application preferences</p>
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
        <span class="switch-label">Show random game button in Home page</span>
      </div>

      <div class="switch-container">
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button
          class="switch"
          class:active={appState.useJpForTitleTime}
          onclick={() =>
            appState.setUseJpForTitleTime(!appState.useJpForTitleTime)}
        >
          <span class="switch-thumb"></span>
        </button>
        <span class="switch-label"
          >Use Japanese for title, Discord presence and time display</span
        >
      </div>

      <div class="switch-container">
        <!-- svelte-ignore a11y_consider_explicit_label -->
        <button
          class="switch"
          class:active={appState.hideNsfwImages}
          onclick={() => appState.setHideNsfwImages(!appState.hideNsfwImages)}
        >
          <span class="switch-thumb"></span>
        </button>
        <span class="switch-label">Hide NSFW images in Home page</span>
      </div>

      <div class="playtime-group">
        <div class="select-container">
          <label for="playtime-mode"
            >Playtime Recording Mode (relaunch game to take effect):</label
          >
          <select
            id="playtime-mode"
            bind:value={playtimeMode}
            onchange={() => appState.setPlaytimeMode(playtimeMode)}
          >
            <option value="classic">Classic</option>
            <option value="ex_static">Pull Data from exSTATic</option>
          </select>
        </div>

        <InfoNote>
          You must use <a
            href={EXSTATIC_GITHUB_URL}
            target="_blank"
            rel="noopener noreferer">this</a
          > version of exSTATic for exSTATic mode to work.
        </InfoNote>
      </div>

      <div class="switch-container">
        <button
          class="reset-button"
          style="background-color: var(--primary);"
          onclick={openLogsDirectory}
        >
          <i class="fa-solid fa-folder-open"></i>
          Open Logs Directory
        </button>
      </div>
    </div>
    <!--
    <div class="settings-section">
      <h2>App Settings</h2>
      <button class="update-button">Check For Updates</button>
      <button class="">DownloadUpdate</button>
    </div>
    -->

    <div class="settings-section danger-section">
      <div class="section-header">
        <h2>Reset Settings</h2>
        <p class="section-description">
          Restore all settings to their default values
        </p>
      </div>
      <button class="reset-button" onclick={resetSettings}>
        <i class="fa-solid fa-rotate-left"></i>
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
    opacity: 0.4;
    font-family: 'SF Mono', 'Monaco', 'Cascadia Code', 'Courier New', monospace;
    font-size: 0.875rem;
    color: var(--secondary-text);
    padding: 0.5rem 0;
  }

  .playtime-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .select-container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .select-container label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--main-text);
    opacity: 0.9;
  }

  .select-container select {
    background-color: rgba(255, 255, 255, 0.03);
    border: 1.5px solid rgba(255, 255, 255, 0.12);
    border-radius: var(--small-radius);
    padding: 0.625rem 0.875rem;
    color: var(--main-text);
    font-size: 0.875rem;
    cursor: pointer;
    transition:
      border-color 0.2s ease,
      box-shadow 0.2s ease,
      background-color 0.2s ease;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='rgba(255, 255, 255, 0.6)' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 0.75rem center;
    padding-right: 2.5rem;
  }

  .select-container select:hover {
    border-color: rgba(255, 255, 255, 0.25);
    background-color: rgba(255, 255, 255, 0.05);
  }

  .select-container select:focus {
    outline: none;
    border-color: var(--primary);
  }
  .container {
    padding: 2rem;
    max-width: 1000px;
    margin: 0 auto;
    min-height: 100vh;
  }

  .content {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 1rem;
    padding: 0;
  }

  .header {
    margin-bottom: 0.5rem;
  }

  .header-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .header h1 {
    font-size: 2.25rem;
    font-weight: 700;
    color: var(--main-text);
    letter-spacing: -0.02em;
  }

  .header p {
    color: var(--secondary-text);
    font-size: 1rem;
    opacity: 0.8;
  }

  .settings-section {
    background-color: var(--accent);
    border-radius: var(--big-radius);
    padding: 1.5rem 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    position: relative;
    overflow: hidden;
    box-shadow:
      0 2px 8px rgba(0, 0, 0, 0.1),
      0 1px 2px rgba(0, 0, 0, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    transition:
      box-shadow 0.3s ease,
      transform 0.2s ease;
  }

  .settings-section:hover {
    box-shadow:
      0 4px 12px rgba(0, 0, 0, 0.15),
      0 2px 4px rgba(0, 0, 0, 0.08);
  }

  .danger-section {
    border: 1.5px solid rgba(239, 68, 68, 0.3);
    background: linear-gradient(
      135deg,
      color-mix(in srgb, var(--accent) 95%, #ef4444 5%) 0%,
      var(--accent) 100%
    );
  }

  .section-header {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    margin-bottom: 0.25rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .section-header h2 {
    font-size: 1.375rem;
    font-weight: 600;
    color: var(--main-text);
    margin: 0;
    letter-spacing: -0.01em;
  }

  .section-description {
    font-size: 0.875rem;
    color: var(--secondary-text);
    opacity: 0.7;
    margin: 0;
  }

  .theme-grid {
    will-change: transform;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 1.25rem;
    contain: layout style;
    padding: 0.5rem 0;
  }

  .theme-item {
    background-color: rgba(255, 255, 255, 0.03);
    border-radius: 10px;
    padding: 1rem;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    border: 2px solid rgba(255, 255, 255, 0.08);
    position: relative;
    overflow: hidden;
    contain: layout paint;
  }

  .theme-item:hover {
    background-color: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .theme-item.active {
    border-color: var(--primary);
    box-shadow: 0 0 0 2px var(--primary);
    background-color: rgba(255, 255, 255, 0.05);
  }

  .theme-preview {
    height: 90px;
    display: flex;
    background-color: var(--preview-bg);
    border-radius: 6px;
    overflow: hidden;
    box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  .preview-sidebar {
    width: 10%;
    height: 100%;
    background-color: var(--preview-accent);
  }

  .preview-content {
    width: 90%;
    height: 100%;
    padding: 10px;
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
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--main-text);
    text-align: center;
    opacity: 0.9;
  }
  .custom-color-section {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 0.25rem;
    padding-top: 0.75rem;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
  }
  .custom-color-section:has(.color-options.visible) {
    gap: 0.75rem;
  }

  .custom-color-section:not(:has(.color-options.visible)) {
    /* new fancy selector */
    gap: 0;
  }
  .switch-container {
    display: flex;
    align-items: center;
    gap: 0.875rem;
    padding: 0.25rem 0;
  }

  .switch {
    position: relative;
    width: 44px;
    height: 24px;
    background-color: color-mix(in srgb, var(--accent), white 10%);
    border-radius: 12px;
    padding: 0;
    border: 1.5px solid rgba(255, 255, 255, 0.15);
    cursor: pointer;
    transition:
      background-color 0.25s cubic-bezier(0.4, 0, 0.2, 1),
      border-color 0.25s ease;
    flex-shrink: 0;
  }

  .switch:hover {
    background-color: color-mix(in srgb, var(--accent), white 15%);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .switch-thumb {
    position: absolute;
    top: 1.5px;
    will-change: transform;
    left: 1.5px;
    width: 19px;
    height: 19px;
    background: linear-gradient(135deg, #ffffff 0%, #f5f5f5 100%);
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .switch.active {
    background: var(--primary);
    border-color: var(--primary);
  }

  .switch.active:hover {
    background: color-mix(in srgb, var(--primary) 90%, white 10%);
  }

  .switch.active .switch-thumb {
    transform: translateX(20px);
  }

  .switch-label {
    font-size: 0.9375rem;
    color: var(--main-text);
    opacity: 0.95;
    line-height: 1.5;
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
    width: 54px;
    height: 54px;
    border: 2px solid rgba(255, 255, 255, 0.15);
    border-radius: 8px;
    cursor: pointer;
    background: none;
    transition: border-color 0.2s ease;
  }

  .color-picker input:hover {
    border-color: rgba(255, 255, 255, 0.3);
  }

  .color-picker span {
    color: var(--main-text);
    font-family: 'SF Mono', 'Monaco', 'Cascadia Code', 'Courier New', monospace;
    font-size: 0.9375rem;
    font-weight: 500;
    opacity: 0.9;
  }

  .color-swatches {
    display: flex;
    flex-wrap: wrap;
    gap: 0.875rem;
    padding: 0.25rem;
  }

  .color-swatch {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    cursor: pointer;
    transition: border-color 0.2s ease;
    border: 2.5px solid transparent;
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
    border-color: rgba(255, 255, 255, 0.3);
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
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--secondary-text);
    opacity: 0.8;
  }

  .preview-button {
    padding: 0.625rem 1.25rem;
    border-radius: var(--small-radius);
    color: white;
    font-weight: 600;
    font-size: 0.9375rem;
    text-align: center;
    width: fit-content;
    border: 1px solid rgba(255, 255, 255, 0.15);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    cursor: default;
    transition: opacity 0.2s ease;
  }

  .preview-button:hover {
    opacity: 0.85;
  }

  .reset-button {
    background-color: #ef4444;
    color: white;
    border: none;
    padding: 0.75rem 1.25rem;
    border-radius: var(--small-radius);
    font-weight: 600;
    font-size: 0.9375rem;
    cursor: pointer;
    width: fit-content;
    display: flex;
    align-items: center;
    gap: 0.625rem;
    transition: filter 0.2s ease;
  }

  .reset-button:hover {
    filter: brightness(0.9);
  }
</style>
