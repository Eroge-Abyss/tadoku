import { invoke } from '@tauri-apps/api/core';

/**
 * @typedef {import('$lib/types').Game} Game
 * @typedef {import('$lib/types').Novel} Novel
 * @typedef {import('$lib/types').Options} Options
 * @typedef {import('$lib/types').CurrentGame} CurrentGame
 */

/**
 * @typedef {Object} ThemeSettings
 * @property {string} theme - The theme ID (default, tokyo-night, etc)
 * @property {string} accentColor - The custom accent color
 * @property {boolean} useCustomColor - Whether to use custom color
 */

/**
 * @property {string} title - The game's title
 * @property {string} exe_path - Path to the game's executable
 * @property {string|null} icon_url - Path to the game's icon
 * @property {boolean} is_pinned - Whether the game is pinned
 * @property {string[]} categories - Game categories/tags
 * @property {number} playtime - Total playtime in seconds
 * @property {string|null} process_name - Process name to track
 */

/**
 * @typedef {Object} GamesList
 * @property {Object.<string, Game>} games - Map of game IDs to game objects
 */

class AppState {
  /**
   * Theme options
   * @type {Array<{id: string, name: string, primary: string, accent: string}>}
   */
  static themes = [
    {
      id: 'default',
      name: 'Default',
      primary: '#3b82f6',
      background: '#1b1b1b',
      accent: '#2a2a2a',
    },
    {
      id: 'tokyo-night',
      name: 'Tokyo Night',
      primary: '#bb9af7',
      background: '#1a1b26',
      accent: '#24283b',
    },
    {
      id: 'sakura',
      name: 'Sakura Pink',
      primary: '#f978b6',
      background: '#1f1d2e',
      accent: '#2d2a3e',
    },
    {
      id: 'ocean-blue',
      name: 'Ocean Blue',
      primary: '#7aa2f7',
      background: '#192330',
      accent: '#24283b',
    },
    {
      id: 'forest-green',
      name: 'Forest Green',
      primary: '#9ece6a',
      background: '#1e2030',ه
      accent: '#282e44',
    },
  ];

  /**
   * Color swatches for custom accent colors
   * @type {string[]}
   */
  static colorSwatches = [
    '#3b82f6', // Blue
    '#bb9af7', // Purple
    '#f978b6', // Pink
    '#9ece6a', // Green
    '#7aa2f7', // Light blue
    '#e0af68', // Orange
    '#f7768e', // Red
    '#ffffff', // White
  ];

  /**
   * @type {Record<string, Game>}
   */
  #gamesList = $state({});

  /**
   * @type {CurrentGame | null}
   */
  #currentGame = $state(null);

  /**
   * @type {ThemeSettings}
   */
  #themeSettings = $state({
    theme: 'default',
    accentColor: '#3b82f6',
    useCustomColor: false,
  });

  constructor() {
    this.loadThemeSettings();
  }

  // Getters and Setters
  get currentGame() {
    return this.#currentGame;
  }

  set currentGame(game) {
    this.#currentGame = game;
  }

  get gamesList() {
    return this.#gamesList;
  }

  get themeSettings() {
    return this.#themeSettings;
  }

  // theme settings methods

  /**
   * Loads theme settings from localstorage
   */
  loadThemeSettings() {
    try {
      const theme = localStorage.getItem('tadoku-theme') || 'default';
      const accentColor =
        localStorage.getItem('tadoku-custom-color') || '#3b82f6';
      const useCustomColor =
        localStorage.getItem('tadoku-use-custom-color') === 'true';

      this.#themeSettings = {
        theme,
        accentColor,
        useCustomColor,
      };

      // Apply theme immediately after loading
      this.applyThemeSettings();
    } catch (error) {
      console.error('Failed to load theme settings:', error);
    }
  }

  /**
   * Updates theme settings and saves them to localStorage
   * @param {Partial<ThemeSettings>} settings - The settings to update
   */
  updateThemeSettings(settings) {
    this.#themeSettings = {
      ...this.#themeSettings,
      ...settings,
    };

    // Save to localStorage
    localStorage.setItem('tadoku-theme', this.#themeSettings.theme);
    localStorage.setItem(
      'tadoku-custom-color',
      this.#themeSettings.accentColor,
    );
    localStorage.setItem(
      'tadoku-use-custom-color',
      this.#themeSettings.useCustomColor.toString(),
    );

    // Apply the updated settings
    this.applyThemeSettings();
  }

  /**
   * Applies the current theme settings to the document
   */
  applyThemeSettings() {
    const theme =
      AppState.themes.find((t) => t.id === this.#themeSettings.theme) ||
      AppState.themes[0];

    // Apply theme colors as CSS variables
    document.documentElement.style.setProperty(
      '--primary',
      this.#themeSettings.useCustomColor
        ? this.#themeSettings.accentColor
        : theme.primary,
    );
    document.documentElement.style.setProperty(
      '--main-background',
      theme.background,
    );
    document.documentElement.style.setProperty('--accent', theme.accent);
    document.documentElement.style.setProperty('--main-text', '#ffffff');
    document.documentElement.style.setProperty('--secondary-text', '#9ca3af');
    document.documentElement.style.setProperty(
      '--main-mauve',
      this.#themeSettings.useCustomColor
        ? this.#themeSettings.accentColor
        : theme.primary,
    );

    // Apply theme class
    document.documentElement.setAttribute(
      'data-theme',
      this.#themeSettings.theme,
    );
  }

  /**
   * Resets theme settings to defaults and clears localStorage
   */
  resetThemeSettings() {
    const defaultSettings = {
      theme: 'default',
      accentColor: '#3b82f6',
      useCustomColor: false,
    };

    this.updateThemeSettings(defaultSettings);

    // Clear localStorage
    localStorage.removeItem('tadoku-theme');
    localStorage.removeItem('tadoku-custom-color');
    localStorage.removeItem('tadoku-use-custom-color');
  }
  async loadGames() {
    this.#gamesList = await invoke('load_games');
    this.sortGames();
  }

  async saveGame(gameId, game, options = { include_characters: true }) {
    await invoke('save_game', { gameId, game, options });
    await this.loadGames();
  }

  loadGame(gameId) {
    return {
      id: gameId,
      ...this.#gamesList[gameId],
    };
  }

  async deleteGame(gameId) {
    await invoke('delete_game', { gameId });
    await this.loadGames();
  }

  async togglePinned(gameId) {
    await invoke('toggle_pin', { gameId });
    await this.loadGames();
  }

  async updateExePath(gameId, newExePath) {
    await invoke('update_exe', { gameId, newExePath });
    await this.loadGames();
  }

  /**
   * Updates the exe path of a game.
  * @param {string[]} categories - The new categories of the gameii
   * @returns {Promise<void>}
   */
  async setGameCategories(gameId, categories) {
    await invoke('set_game_categories', { gameId, categories });
    await this.loadGames();
  }

  async updateGameProcessPath(gameId, newProcessPath) {
    await invoke('update_process', { gameId, newProcessPath });
    await this.loadGames();
  }

  /**
   * Updates the exe path of a game.
   * @param {string} gameId - The unique identifier for the game.
   * @param {string} newProcessPath - The new process path of the game
   * @returns {Promise<void>}
   */

  async startGame(gameId) {
    await invoke('open_game', { gameId });
  }

  async closeGame() {
    await invoke('close_game', {});
  }

  sortGames() {
    const sortedEntries = Object.entries(this.#gamesList).sort(
      ([, a], [, b]) => {
        if (b.playtime !== a.playtime) return b.playtime - a.playtime;
        return a.title.localeCompare(b.title);
      },
    );
    this.#gamesList = Object.fromEntries(sortedEntries);
  }
}

export const appState = new AppState();
