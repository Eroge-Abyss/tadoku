import { invoke } from '@tauri-apps/api/core';

/**
 * @typedef {import('$lib/types').Game} Game
 * @typedef {import('$lib/types').Novel} Novel
 * @typedef {import('$lib/types').Options} Options
 * @typedef {import('$lib/types').CurrentGame} CurrentGame
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

  /**
   * Loads the list of games from the backend.
   * @returns {Promise<void>}
   */
  async loadGames() {
    this.#gamesList = await invoke('load_games');
    this.sortGames();
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

    await this.loadGames();
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

    await this.loadGames();
  }

  /**
   * Toggles the pinned status of a game.
   * @param {string} gameId - The unique identifier for the game.
   * @returns {Promise<void>}
   */
  async togglePinned(gameId) {
    await invoke('toggle_pin', { gameId });

    await this.loadGames();
  }

  /**
   * Updates the exe path of a game.
   * @param {string} gameId - The unique identifier for the game.
   * @param {string} newExePath - The new exe path of the game
   * @returns {Promise<void>}
   */
  async updateExePath(gameId, newExePath) {
    await invoke('update_exe', { gameId, newExePath });

    await this.loadGames();
  }

  /**
   * Updates the exe path of a game.
   * @param {string} gameId - The unique identifier for the game.
   * @param {string[]} categories - The new exe path of the game
   * @returns {Promise<void>}
   */
  async setGameCategories(gameId, categories) {
    await invoke('set_game_categories', { gameId, categories });

    await this.loadGames();
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
   * Sorts the games list by title.
   */
  sortGames() {
    const sortedEntries = Object.entries(this.#gamesList).sort(
      ([, a], [, b]) => {
        if (b.playtime !== a.playtime) {
          return b.playtime - a.playtime;
        }
        return a.title.localeCompare(b.title);
      },
    );

    this.#gamesList = Object.fromEntries(sortedEntries);
  }
}

export const appState = new AppState();
