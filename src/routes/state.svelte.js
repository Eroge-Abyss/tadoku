import { invoke } from '@tauri-apps/api/core';
import { THEMES, DEFAULT_THEME_SETTINGS } from '../themeConstants.js';
/**
 * @typedef {import('$lib/types').Game} Game
 * @typedef {import('$lib/types').Novel} Novel
 * @typedef {import('$lib/types').Options} Options
 * @typedef {import('$lib/types').CurrentGame} CurrentGame
 * @typedef {import('$lib/types').ThemeSettings} ThemeSettings
 * @typedef {import('$lib/types').SortOrder} SortOrder
 */

class AppState {
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
  #themeSettings = $state({ ...DEFAULT_THEME_SETTINGS });

  /**
   * @type {SortOrder | null}
   */
  #sortOrder = $state(null);

  constructor() {
    this.loadThemeSettings();
    this.loadSortOrder();
  }

  // Getters and Setters
  get currentGame() {
    return this.#currentGame;
  }

  set currentGame(game) {
    this.#currentGame = game;
  }

  /**
   * Gets the list of games.
   * @returns {Record<string, Game>}
   */
  get gamesList() {
    return this.#gamesList;
  }

  get themeSettings() {
    return this.#themeSettings;
  }

  get sortOrder() {
    return this.#sortOrder;
  }

  async loadThemeSettings() {
    try {
      const { theme, accent_color, use_custom_accent } =
        await invoke('get_theme_settings');

      this.#themeSettings = {
        theme,
        accentColor: accent_color,
        useCustomColor: use_custom_accent,
      };

      this.applyThemeSettings();
    } catch (error) {
      console.error('Failed to load theme settings:', error);
    }
  }

  /**
   * Updates theme settings and saves them to localStorage
   * @param {Partial<ThemeSettings>} settings - The settings to update
   */
  async updateThemeSettings(settings) {
    this.#themeSettings = {
      ...this.#themeSettings,
      ...settings,
    };

    console.log(this.#themeSettings);

    await invoke('set_theme_settings', {
      themeSettings: {
        accent_color: this.#themeSettings.accentColor,
        theme: this.#themeSettings.theme,
        use_custom_accent: this.#themeSettings.useCustomColor,
      },
    });

    this.applyThemeSettings();
  }

  /**
   * Applies the current theme settings to the document
   */
  applyThemeSettings() {
    const theme =
      THEMES.find((t) => t.id === this.#themeSettings.theme) || THEMES[0];

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

    document.documentElement.setAttribute(
      'data-theme',
      this.#themeSettings.theme,
    );
  }

  /**
   * Resets theme settings to defaults and clears localStorage
   */
  resetThemeSettings() {
    this.#themeSettings = { ...DEFAULT_THEME_SETTINGS };
    this.updateThemeSettings(DEFAULT_THEME_SETTINGS);
  }

  /**
   * Loads the list of games from the backend and sorts them.
   * @returns {Promise<void>}
   */
  async loadGames() {
    await this.refreshGamesList();
    this.sortGames();
  }

  /**
   * Loads the games list from the backend.
   * @returns {Promise<void>}
   */
  async refreshGamesList() {
    this.#gamesList = await invoke('load_games');
  }

  /**
   * Saves a game to the backend.
   * @param {string} gameId - The unique identifier for the game.
   * @param {Game} game - The game object to save.
   * @param {Options} options - The game object to save.
   * @returns {Promise<void>}
   */
  async saveGame(gameId, game, options = { include_characters: true }) {
    await invoke('save_game', {
      gameId,
      game,
      options,
    });

    await this.refreshGamesList();
  }

  /**
   * Loads a specific game by its ID.
   * @param {string} gameId - The unique identifier for the game.
   * @returns {Novel | undefined} The game object, or undefined if not found.
   */
  loadGame(gameId) {
    return {
      id: gameId,
      ...this.#gamesList[gameId],
    };
  }

  /**
   * Deletes a game from the backend.
   * @param {string} gameId - The unique identifier for the game.
   * @returns {Promise<void>}
   */
  async deleteGame(gameId) {
    await invoke('delete_game', { gameId });

    await this.refreshGamesList();
  }

  /**
   * Toggles the pinned status of a game.
   * @param {string} gameId - The unique identifier for the game.
   * @returns {Promise<void>}
   */
  async togglePinned(gameId) {
    await invoke('toggle_pin', { gameId });

    await this.refreshGamesList();
  }

  /**
   * Updates the exe path of a game.
   * @param {string} gameId - The unique identifier for the game.
   * @param {string} newExePath - The new exe path of the game
   * @returns {Promise<void>}
   */
  async updateExePath(gameId, newExePath) {
    await invoke('update_exe', { gameId, newExePath });

    await this.refreshGamesList();
  }

  /**
   * Updates the exe path of a game.
   * @param {string} gameId - The unique identifier for the game.
   * @param {string[]} categories - The new categories of the game
   * @returns {Promise<void>}
   */
  async setGameCategories(gameId, categories) {
    await invoke('set_game_categories', { gameId, categories });

    await this.refreshGamesList();
  }

  /**
   * Updates the exe path of a game.
   * @param {string} gameId - The unique identifier for the game.
   * @param {string} newProcessPath - The new process path of the game
   * @returns {Promise<void>}
   */
  async updateGameProcessPath(gameId, newProcessPath) {
    await invoke('update_process', {
      gameId,
      newProcessPath: newProcessPath,
    });

    await this.refreshGamesList();
  }

  /**
   * Opens a game in a child process
   * @param {string} gameId - The unique identifier for the game.
   * @returns {Promise<void>}
   */
  async startGame(gameId) {
    await invoke('open_game', { gameId });
  }

  /**
   * Closes the currently opened game
   * @returns {Promise<void>}
   */
  async closeGame() {
    await invoke('close_game', {});
  }

  /**
   * Sorts the games list by playtime.
   */
  async sortGames() {
    const sortOrder = this.#sortOrder;
    const sortedEntries =
      sortOrder === 'last_played'
        ? this.sortByLastPlayed()
        : sortOrder === 'playtime'
          ? this.sortByPlaytime()
          : this.sortByTitle();

    this.#gamesList = Object.fromEntries(sortedEntries);
  }

  /**
   * Opens a game in a child process
   * @param {SortOrder} sortOrder - The unique identifier for the game.
   * @returns {Promise<void>}
   */
  async setSortOrder(sortOrder) {
    await invoke('set_sort_order', { sortOrder });
    await this.refreshGamesList();
  }

  async loadSortOrder() {
    this.#sortOrder = await invoke('get_sort_order');
  }

  sortByPlaytime() {
    return Object.entries(this.#gamesList).sort(([, a], [, b]) => {
      if (b.playtime !== a.playtime) {
        return b.playtime - a.playtime;
      }
      return a.title.localeCompare(b.title);
    });
  }

  sortByTitle() {
    return Object.entries(this.#gamesList).sort(([, a], [, b]) =>
      a.title.localeCompare(b.title),
    );
  }

  sortByLastPlayed() {
    return Object.entries(this.#gamesList).sort(
      ([, a], [, b]) =>
        (b.last_played || Infinity) - (a.last_played || Infinity),
    );
  }
}

export const appState = new AppState();
