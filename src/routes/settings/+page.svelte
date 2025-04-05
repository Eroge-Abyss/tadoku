<script>
  import { onMount } from 'svelte';
  import { appState } from '../state.svelte.js';

  // use themes constants from the appState
  const themes = appState.constructor.themes;
  const colorSwatches = appState.constructor.colorSwatches;

  let selectedTheme = $state(appState.themeSettings.theme);
  let customColor = $state(appState.themeSettings.accentColor);
  let useCustomColor = $state(appState.themeSettings.useCustomColor);
  onMount(() => {
    $effect(() => {
      selectedTheme = appState.themeSettings.theme;
      customColor = appState.themeSettings.accentColor;
      useCustomColor = appState.themeSettings.useCustomColor;
    });
  });

  //add function to get the app version from tauri
  //static version value for now
  const appVersion = 'v1.0.4';
  function selectTheme(themeId) {
    selectedTheme = themeId;
    appState.updateThemeSettings({ theme: themeId });
  }

  function toggleCustomColor() {
    useCustomColor = !useCustomColor;
    appState.updateThemeSettings({ useCustomColor });
  }
  function selectColor(color) {
    customColor = color;
    useCustomColor = true;
    appState.updateThemeSettings({
      accentColor: color,
      useCustomColor: true,
    });
  }

  function handleColorInput(event) {
    customColor = event.target.value;
    useCustomColor = true;
    appState.updateThemeSettings({
      accentColor: customColor,
      useCustomColor: true,
    });
  }
  //this reset function currently resets settings for themes only.
  function resetSettings() {
    appState.resetThemeSettings();

    // Update local state vars
    selectedTheme = appState.themeSettings.theme;
    customColor = appState.themeSettings.accentColor;
    useCustomColor = appState.themeSettings.useCustomColor;
  }
</script>

<div class="container">
  <div class="content">
    <div class="header">
      <h1>Settings</h1>
      <p>Customize your Tadoku experience</p>
    </div>

    <div class="settings-section">
      <h2>Theme Selection</h2>
      <div class="theme-grid">
        {#each themes as theme}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="theme-item"
            class:active={selectedTheme === theme.id}
            style="--preview-primary: {theme.primary}; --preview-bg: {theme.background}; --preview-accent: {theme.accent};"
            onclick={() => selectTheme(theme.id)}
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

        <div class="color-options">
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
            {#each colorSwatches as color}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                class="color-swatch"
                style="background-color: {color};"
                class:active={useCustomColor && customColor === color}
                onclick={() => selectColor(color)}
              ></div>
            {/each}
          </div>
        </div>

        <div class="preview">
          <div class="preview-title">Preview</div>
          <div
            class="preview-button"
            style="background-color: {useCustomColor
              ? customColor
              : themes.find((t) => t.id === selectedTheme).primary}"
          >
            Button
          </div>
        </div>
      </div>
    </div>
    <!--
    <div class="settings-section">
      <h2>App Settings</h2>
      <div class="switch-container">
        <button class="switch" class:active={useCustomColor}>
          <span class="switch-thumb"></span>
        </button>
        <span class="switch-label">disable discord presence</span>
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
      Tadoku {appVersion}
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
    border-radius: 12px;
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
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.05) 0%,
      rgba(255, 255, 255, 0.02) 50%,
      transparent 100%
    );
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
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 1rem;
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
  }

  .theme-item::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 80%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 255, 255, 0.05),
      transparent
    );
    transition: 0.3s ease-in;
  }

  .theme-item:hover::before {
    left: 100%;
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
    border-radius: 4px;
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
    border-radius: 4px;
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
    gap: 1.5rem;
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
    border-radius: 11px;
    padding: 0;
    border: 1px solid rgba(255, 255, 255, 0.1);
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .switch.active {
    background: var(--primary);
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
  }

  .switch-thumb {
    position: absolute;
    top: 1px;
    left: 1px;
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
    font-size: 14px;
    color: var(--main-text);
  }

  .color-options {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
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
    margin-top: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .preview-title {
    font-size: 14px;
    color: var(--secondary-text);
  }

  .preview-button {
    padding: 0.5rem 1rem;
    border-radius: 4px;
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
    border-radius: 4px;
    font-weight: 500;
    cursor: pointer;
    width: fit-content;
    transition: background-color 0.2s;
  }

  .reset-button:hover {
    background-color: #dc2626;
  }
</style>
