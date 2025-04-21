import { invoke } from "@tauri-apps/api/core";
import { themes, defaultThemeSettings } from "../themeConstants.js";
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
	#themeSettings = $state({ ...defaultThemeSettings });

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

	async loadThemeSettings() {
		try {
			const { theme, accent_color, use_custom_accent } =
				await invoke("get_theme_settings");

			this.#themeSettings = {
				theme,
				accentColor: accent_color,
				useCustomColor: use_custom_accent,
			};

			this.applyThemeSettings();
		} catch (error) {
			console.error("Failed to load theme settings:", error);
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

		await invoke("set_theme_settings", {
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
			themes.find((t) => t.id === this.#themeSettings.theme) ||
			themes[0];

		document.documentElement.style.setProperty(
			"--primary",
			this.#themeSettings.useCustomColor
				? this.#themeSettings.accentColor
				: theme.primary,
		);
		document.documentElement.style.setProperty(
			"--main-background",
			theme.background,
		);
		document.documentElement.style.setProperty("--accent", theme.accent);
		document.documentElement.style.setProperty("--main-text", "#ffffff");
		document.documentElement.style.setProperty("--secondary-text", "#9ca3af");
		document.documentElement.style.setProperty(
			"--main-mauve",
			this.#themeSettings.useCustomColor
				? this.#themeSettings.accentColor
				: theme.primary,
		);

		document.documentElement.setAttribute(
			"data-theme",
			this.#themeSettings.theme,
		);
	}

	/**
	 * Resets theme settings to defaults and clears localStorage
	 */
	resetThemeSettings() {
		this.#themeSettings = { ...defaultThemeSettings };
    	this.updateThemeSettings(defaultThemeSettings);
	}

	async loadGames() {
		this.#gamesList = await invoke("load_games");
		this.sortGames();
	}

	async saveGame(gameId, game, options = { include_characters: true }) {
		await invoke("save_game", { gameId, game, options });
		await this.loadGames();
	}

	loadGame(gameId) {
		return {
			id: gameId,
			...this.#gamesList[gameId],
		};
	}

	async deleteGame(gameId) {
		await invoke("delete_game", { gameId });
		await this.loadGames();
	}

	async togglePinned(gameId) {
		await invoke("toggle_pin", { gameId });
		await this.loadGames();
	}

	async updateExePath(gameId, newExePath) {
		await invoke("update_exe", { gameId, newExePath });
		await this.loadGames();
	}

	/**
	 * Updates the exe path of a game.
	 * @param {string[]} categories - The new categories of the gameii
	 * @returns {Promise<void>}
	 */
	async setGameCategories(gameId, categories) {
		await invoke("set_game_categories", { gameId, categories });
		await this.loadGames();
	}

	async updateGameProcessPath(gameId, newProcessPath) {
		await invoke("update_process", { gameId, newProcessPath });
		await this.loadGames();
	}

	/**
	 * Updates the exe path of a game.
	 * @param {string} gameId - The unique identifier for the game.
	 * @param {string} newProcessPath - The new process path of the game
	 * @returns {Promise<void>}
	 */

	async startGame(gameId) {
		await invoke("open_game", { gameId });
	}

	async closeGame() {
		await invoke("close_game", {});
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
