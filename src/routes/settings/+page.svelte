<script>
  import { onMount } from 'svelte';
  import { appState } from '../state.svelte.js';

  // use themes constants from the appState
  // @ts-ignore
  const themes = appState.constructor.themes;
  // @ts-ignore
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

  // @ts-ignore
  function selectTheme(themeId) {
    selectedTheme = themeId;
    appState.updateThemeSettings({ theme: themeId });
  }

  function toggleCustomColor() {
    useCustomColor = !useCustomColor;
    appState.updateThemeSettings({ useCustomColor });
  }

  // @ts-ignore
  function selectColor(color) {
    customColor = color;
    useCustomColor = true;
    appState.updateThemeSettings({
      accentColor: color,
      useCustomColor: true,
    });
  }

  // @ts-ignore
  function handleColorInput(event) {
    customColor = event.target.value;
    useCustomColor = true;
    appState.updateThemeSettings({
      accentColor: customColor,
      useCustomColor: true,
    });
  }

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

    <div class="settings-section">
      <h2>Reset Settings</h2>
      <button class="reset-button" onclick={resetSettings}>
        Reset to Default Settings
      </button>
    </div>
  </div>
</div>

<style>
  .container {
    padding: 25px;
  }
  .content {
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 2rem;
  }

  .header {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .header h1 {
    font-size: 2rem;
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
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .settings-section h2 {
    font-size: 1.5rem;
    font-weight: 500;
    color: var(--main-text);
    margin-bottom: 1rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    padding-bottom: 0.5rem;
  }
  
  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 1rem;
  }

  .theme-item {
    background-color: #313131;
    border-radius: 8px;
    padding: 1rem;
    cursor: pointer;
    transition:
      transform 0.2s,
      box-shadow 0.2s;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    border: 2px solid transparent;
  }

  .theme-item:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  }

  .theme-item.active {
    border-color: var(--primary);
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
  }
  .custom-color-section {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .switch-container {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .switch {
    position: relative;
    width: 40px;
    height: 22px;
    background-color: #444;
    border-radius: 11px;
    padding: 0;
    border: none;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .switch.active {
    background-color: var(--primary);
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
    font-size: 14px;
    color: var(--main-text);
  }

  .color-options {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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
    font-size: 14px;
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
