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

/**
 * Manages the global application state, including the games list,
 * currently active game, theme settings, and sorting preferences.
 * Interacts with the Tauri backend via `invoke` calls.
 */
class AppState {
  /**
   * The list of games, keyed by game ID.
   * @type {Record<string, Game>}
   */
  #gamesList = $state({});

  /**
   * The currently selected or active game.
   * @type {CurrentGame | null}
   */
  #currentGame = $state(null);

  /**
   * The user's theme settings.
   * @type {ThemeSettings}
   */
  #themeSettings = $state({ ...DEFAULT_THEME_SETTINGS });

  /**
   * The current sort order for the games list.
   * @type {SortOrder | null}
   */
  #sortOrder = $state(null);

  /**
   * Whether Discord presence should be disabled for NSFW games.
   * @type {boolean}
   */
  #disablePresenceOnNsfw = $state(true);

  /**
   * Whether the random game picker button should be shown.
   * @type {boolean}
   */
  #showRandomButton = $state(false);

  /**
   * Creates an instance of AppState and loads initial settings.
   */
  constructor() {
    this.loadThemeSettings();
    this.loadSortOrder();
    this.loadShowRandomButton();
    this.loadDisablePresenceOnNsfw();
  }

  // --- Getters ---

  /**
   * Gets the currently active game.
   * @returns {CurrentGame | null}
   */
  get currentGame() {
    return this.#currentGame;
  }

  /**
   * Gets whether Discord presence is disabled for NSFW games.
   * @returns {boolean}
   */
  get disablePresenceOnNsfw() {
    return this.#disablePresenceOnNsfw;
  }

  /**
   * Gets the state of the random game picker button.
   * @returns {boolean}
   */
  get showRandomButton() {
    return this.#showRandomButton;
  }

  /**
   * Gets the list of games.
   * @returns {Record<string, Game>}
   */
  get gamesList() {
    return this.#gamesList;
  }

  /**
   * Gets the current theme settings.
   * @returns {ThemeSettings}
   */
  get themeSettings() {
    return this.#themeSettings;
  }

  /**
   * Gets the current sort order for the games list.
   * @returns {SortOrder | null}
   */
  get sortOrder() {
    return this.#sortOrder;
  }

  // --- Setters ---

  /**
   * Sets the currently active game.
   * @param {CurrentGame | null} game - The game to set as current.
   */
  set currentGame(game) {
    this.#currentGame = game;
  }

  // --- Initialization / Loading Methods ---

  /**
   * Loads the list of games from the backend and sorts them.
   * This is typically called on application startup.
   * @returns {Promise<void>}
   */
  async loadGames() {
    await this.refreshGamesList();
  }

  /**
   * Loads the games list from the backend and applies the current sort order.
   * @returns {Promise<void>}
   */
  async refreshGamesList() {
    try {
      this.#gamesList = await invoke('load_games');
      this.sortGames();
    } catch (error) {
      console.error('Failed to load games list:', error);
    }
  }

  /**
   * Loads the theme settings from the backend and applies them.
   * @returns {Promise<void>}
   */
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
   * Loads the sort order from the backend.
   * @returns {Promise<void>}
   */
  async loadSortOrder() {
    try {
      this.#sortOrder = await invoke('get_sort_order');
    } catch (error) {
      console.error('Failed to load sort order:', error);
    }
  }

  /**
   * Loads the disable presence on NSFW setting from the backend.
   * @returns {Promise<void>}
   */
  async loadDisablePresenceOnNsfw() {
    try {
      this.#disablePresenceOnNsfw = await invoke('get_nsfw_presence_status');
    } catch (error) {
      console.error('Failed to load disable presence on NSFW:', error);
    }
  }

  /**
   * Loads the show random button setting from the backend.
   * @returns {Promise<void>}
   */
  async loadShowRandomButton() {
    try {
      this.#showRandomButton = await invoke('get_show_random_picker');
    } catch (error) {
      console.error('Failed to load show random button setting:', error);
    }
  }

  // --- Game Management Methods ---

  /**
   * Saves a game to the backend and refreshes the games list.
   * @param {string} gameId - The unique identifier for the game.
   * @param {Game} game - The game object to save.
   * @param {Options} [options={ include_characters: true }] - Options for saving (e.g., include characters).
   * @returns {Promise<void>}
   */
  async saveGame(gameId, game, options = { include_characters: true }) {
    try {
      await invoke('save_game', {
        gameId,
        game,
        options,
      });
      await this.refreshGamesList();
    } catch (error) {
      console.error(`Failed to save game ${gameId}:`, error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Loads a specific game by its ID from the currently loaded list.
   * Note: This does not fetch from the backend; it uses the cached list.
   * @param {string} gameId - The unique identifier for the game.
   * @returns {Novel | undefined} The novel object (Game with ID), or undefined if not found.
   */
  loadGame(gameId) {
    const game = this.#gamesList[gameId];
    if (game) {
      return {
        id: gameId,
        ...game,
      };
    }
    return undefined;
  }

  /**
   * Deletes a game from the backend and refreshes the games list.
   * @param {string} gameId - The unique identifier for the game.
   * @returns {Promise<void>}
   */
  async deleteGame(gameId) {
    try {
      await invoke('delete_game', { gameId });
      await this.refreshGamesList();
    } catch (error) {
      console.error(`Failed to delete game ${gameId}:`, error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Toggles the pinned status of a game in the backend and refreshes the list.
   * @param {string} gameId - The unique identifier for the game.
   * @returns {Promise<void>}
   */
  async togglePinned(gameId) {
    try {
      await invoke('toggle_pin', { gameId });
      await this.refreshGamesList();
    } catch (error) {
      console.error(`Failed to toggle pin for game ${gameId}:`, error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Updates the executable path of a game in the backend and refreshes the list.
   * @param {string} gameId - The unique identifier for the game.
   * @param {string} newExePath - The new executable path.
   * @returns {Promise<void>}
   */
  async updateExePath(gameId, newExePath) {
    try {
      await invoke('update_exe', { gameId, newExePath });
      await this.refreshGamesList();
    } catch (error) {
      console.error(`Failed to update exe path for game ${gameId}:`, error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Updates the categories of a game in the backend and refreshes the list.
   * @param {string} gameId - The unique identifier for the game.
   * @param {string[]} categories - The new list of categories.
   * @returns {Promise<void>}
   */
  async setGameCategories(gameId, categories) {
    try {
      await invoke('set_game_categories', { gameId, categories });
      await this.refreshGamesList();
    } catch (error) {
      console.error(`Failed to set categories for game ${gameId}:`, error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Updates the process path of a game in the backend and refreshes the list.
   * This path is used to detect if the game is running.
   * @param {string} gameId - The unique identifier for the game.
   * @param {string} newProcessPath - The new process path.
   * @returns {Promise<void>}
   */
  async updateGameProcessPath(gameId, newProcessPath) {
    try {
      await invoke('update_process', {
        gameId,
        newProcessPath: newProcessPath,
      });
      await this.refreshGamesList();
    } catch (error) {
      console.error(`Failed to update process path for game ${gameId}:`, error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Opens a game in a child process using the configured executable path.
   * @param {string} gameId - The unique identifier for the game.
   * @returns {Promise<void>}
   */
  async startGame(gameId) {
    try {
      await invoke('open_game', { gameId });
    } catch (error) {
      console.error(`Failed to start game ${gameId}:`, error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Closes the currently opened game process.
   * @returns {Promise<void>}
   */
  async closeGame() {
    try {
      await invoke('close_game', {});
    } catch (error) {
      console.error('Failed to close game:', error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  // --- Theme Management Methods ---

  /**
   * Updates theme settings in the backend and applies them.
   * @param {Partial<ThemeSettings>} settings - The settings to update.
   * @returns {Promise<void>}
   */
  async updateThemeSettings(settings) {
    try {
      this.#themeSettings = {
        ...this.#themeSettings,
        ...settings,
      };

      // console.log(this.#themeSettings); // Removed this console log as part of cleanup

      await invoke('set_theme_settings', {
        themeSettings: {
          accent_color: this.#themeSettings.accentColor,
          theme: this.#themeSettings.theme,
          use_custom_accent: this.#themeSettings.useCustomColor,
        },
      });

      this.applyThemeSettings();
    } catch (error) {
      console.error('Failed to update theme settings:', error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Applies the current theme settings to the document's CSS variables.
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
      '--main-mauve', // Note: This might be a specific theme variable, confirm usage.
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
   * Resets theme settings to defaults, saves them to the backend, and applies them.
   * @returns {Promise<void>}
   */
  async resetThemeSettings() {
    this.#themeSettings = { ...DEFAULT_THEME_SETTINGS };
    // Re-call updateThemeSettings to save and apply defaults via the standard method
    await this.updateThemeSettings(DEFAULT_THEME_SETTINGS);
  }

  // --- Sorting and UI State Methods ---

  /**
   * Sorts the internal games list based on the current sort order setting.
   */
  sortGames() {
    const sortOrder = this.#sortOrder;
    let sortedEntries;

    switch (sortOrder) {
      case 'last_played':
        sortedEntries = this.sortByLastPlayed();
        break;
      case 'playtime':
        sortedEntries = this.sortByPlaytime();
        break;
      case 'title':
      default: // Default to sorting by title
        sortedEntries = this.sortByTitle();
        break;
    }

    // Update the state to trigger reactivity
    this.#gamesList = Object.fromEntries(sortedEntries);
  }

  /**
   * Sets the sort order, saves it to the backend, and re-sorts the games list.
   * @param {SortOrder} sortOrder - The desired sort order ('title', 'playtime', 'last_played').
   * @returns {Promise<void>}
   */
  async setSortOrder(sortOrder) {
    try {
      await invoke('set_sort_order', { sortOrder });
      this.#sortOrder = sortOrder;
      this.sortGames(); // Sort immediately after setting
    } catch (error) {
      console.error('Failed to set sort order:', error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Sets whether the random game picker button should be shown and saves the setting to the backend.
   * @param {boolean} to - The desired state (true to show, false to hide).
   * @returns {Promise<void>}
   */
  async setShowRandomButton(to) {
    try {
      this.#showRandomButton = to;
      await invoke('set_show_random_picker', { to });
    } catch (error) {
      console.error('Failed to set show random button setting:', error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  /**
   * Sets whether Discord presence should be disabled for NSFW games and saves the setting to the backend.
   * @param {boolean} to - The desired state (true to disable, false to enable).
   * @returns {Promise<void>}
   */
  async setDisablePresenceOnNsfw(to) {
    try {
      this.#disablePresenceOnNsfw = to;
      await invoke('set_nsfw_presence_status', { to });
    } catch (error) {
      console.error('Failed to set NSFW presence status:', error);
      throw error; // Re-throw to allow UI to handle
    }
  }

  // --- Private/Helper Sorting Methods ---

  /**
   * Sorts the games list entries by playtime (descending), then by title (ascending).
   * @private
   * @returns {[string, Game][]} Sorted entries.
   */
  sortByPlaytime() {
    return Object.entries(this.#gamesList).sort(([, a], [, b]) => {
      // Sort by playtime descending
      if (b.playtime !== a.playtime) {
        return (b.playtime || 0) - (a.playtime || 0); // Treat undefined/null playtime as 0
      }
      // If playtime is the same, sort by title ascending
      return a.title.localeCompare(b.title);
    });
  }

  /**
   * Sorts the games list entries by title (ascending).
   * @private
   * @returns {[string, Game][]} Sorted entries.
   */
  sortByTitle() {
    return Object.entries(this.#gamesList).sort(([, a], [, b]) =>
      a.title.localeCompare(b.title),
    );
  }

  /**
   * Sorts the games list entries by last played timestamp (descending), then by title (ascending).
   * @private
   * @returns {[string, Game][]} Sorted entries.
   */
  sortByLastPlayed() {
    return Object.entries(this.#gamesList).sort(([, a], [, b]) => {
      // Sort by last_played descending
      if (b.last_played !== a.last_played) {
        // Treat undefined/null last_played as 0 for comparison
        return (b.last_played || 0) - (a.last_played || 0);
      }
      // If last_played is the same, sort by title ascending
      return a.title.localeCompare(b.title);
    });
  }
}

export const appState = new AppState();
