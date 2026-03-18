import { settingsStore } from '$lib/stores/settings.svelte';
import * as gamesService from '$lib/services/games.service';
import { toast } from 'svelte-sonner';
import type { Game, GameDto, Novel, Options, ProcessItem } from '$lib/types';
import { getAvailable } from '$lib/util';

class GamesStore {
  #games: Record<string, Game> = $state({});

  get list(): Record<string, Game> {
    return this.#games;
  }

  get sorted(): Record<string, Game> {
    const entries = Object.entries(this.#games);

    const compareFn =
      settingsStore.sortOrder === 'playtime'
        ? this.#byPlaytime
        : settingsStore.sortOrder === 'last_played'
          ? this.#byLastPlayed
          : this.#byTitle;

    entries.sort((a, b) => compareFn(a, b));

    return Object.fromEntries(entries);
  }

  get filtered(): Record<string, Game> {
    const sortedEntries = Object.entries(this.sorted);
    if (settingsStore.selectedCategories.length === 0)
      return Object.fromEntries(sortedEntries);

    return Object.fromEntries(
      sortedEntries.filter(
        ([, g]) =>
          g.categories.some((c) =>
            settingsStore.selectedCategories.includes(c),
          ) ||
          (g.categories.length === 0 &&
            settingsStore.selectedCategories.includes('Uncategorized')),
      ),
    );
  }

  #byPlaytime = ([_a, a]: [string, Game], [_b, b]: [string, Game]): number => {
    return b.playtime - a.playtime || this.#byTitle([_a, a], [_b, b]);
  };

  #byLastPlayed = (
    [_a, a]: [string, Game],
    [_b, b]: [string, Game],
  ): number => {
    return (
      (b.last_played || 0) - (a.last_played || 0) ||
      this.#byTitle([_a, a], [_b, b])
    );
  };

  #byTitle = ([, a]: [string, Game], [, b]: [string, Game]): number => {
    return a.title.localeCompare(b.title);
  };

  getById(id: string): Novel | undefined {
    const game = this.#games[id];
    return game ? { id, ...game } : undefined;
  }

  async init(): Promise<void> {
    await this.refresh();
  }

  async refresh(): Promise<void> {
    try {
      this.#games = await gamesService.load();
    } catch (error) {
      console.error('Failed to load games list:', error);
    }
  }

  async saveGame(
    gameId: string,
    game: GameDto,
    options: Options = { include_characters: true },
  ): Promise<void> {
    try {
      await gamesService.save(gameId, game, options);
      await this.refresh();
    } catch (error) {
      console.error(`Failed to save game ${gameId}:`, error);
      toast.error(`Failed to save game: ${error}`);
      throw error;
    }
  }

  async deleteGame(gameId: string): Promise<void> {
    try {
      await gamesService.remove(gameId);
      await this.refresh();
    } catch (error) {
      console.error(`Failed to delete game ${gameId}:`, error);
      toast.error(`Failed to delete game: ${error}`);
      throw error;
    }
  }

  async togglePinned(gameId: string): Promise<void> {
    try {
      await gamesService.togglePin(gameId);
      await this.refresh();
    } catch (error) {
      console.error(`Failed to toggle pin for game ${gameId}:`, error);
      toast.error(`Failed to toggle pin: ${error}`);
      throw error;
    }
  }

  async resetStats(gameId: string): Promise<void> {
    try {
      await gamesService.resetStats(gameId);
      await this.refresh();
    } catch (error) {
      console.error(`Failed to reset stats for game ${gameId}:`, error);
      toast.error(`Failed to reset stats: ${error}`);
      throw error;
    }
  }

  async setCharacters(gameId: string): Promise<void> {
    try {
      await gamesService.setCharacters(gameId);
      await this.refresh();
    } catch (error) {
      console.error(`Failed to set characters for game ${gameId}:`, error);
      toast.error(`Failed to set characters: ${error}`);
      throw error;
    }
  }

  async updateExePath(gameId: string, newExePath: string): Promise<void> {
    try {
      await gamesService.updateExePath(gameId, newExePath);
      await this.refresh();
    } catch (error) {
      console.error(`Failed to update exe path for game ${gameId}:`, error);
      toast.error(`Failed to update exe path: ${error}`);
      throw error;
    }
  }

  async setGameCategories(gameId: string, categories: string[]): Promise<void> {
    try {
      await gamesService.setCategories(gameId, categories);
      await this.refresh();
    } catch (error) {
      console.error(`Failed to set categories for game ${gameId}:`, error);
      toast.error(`Failed to set categories: ${error}`);
      throw error;
    }
  }

  async updateGameProcessPath(
    gameId: string,
    newProcessPath: string,
  ): Promise<void> {
    try {
      await gamesService.updateProcessPath(gameId, newProcessPath);
      await this.refresh();
    } catch (error) {
      console.error(`Failed to update process path for game ${gameId}:`, error);
      toast.error(`Failed to update process path: ${error}`);
      throw error;
    }
  }

  async startGame(gameId: string): Promise<void> {
    try {
      await gamesService.openGame(gameId);
    } catch (error) {
      console.error(`Failed to start game ${gameId}:`, error);
      toast.error(`Failed to start game: ${error}`);
      throw error;
    }
  }

  async closeGame(): Promise<void> {
    try {
      await gamesService.closeGame();
    } catch (error) {
      console.error('Failed to close game:', error);
      toast.error(`Failed to close game: ${error}`);
      throw error;
    }
  }

  async getActiveWindows(): Promise<ProcessItem[]> {
    try {
      return await gamesService.getActiveWindows();
    } catch (error) {
      console.error('Failed to get active windows:', error);
      toast.error(`Failed to get active windows: ${error}`);
      return [];
    }
  }

  async setGameNotes(gameId: string, notes: string): Promise<void> {
    try {
      await gamesService.setNotes(gameId, notes);
    } catch (error) {
      console.error(`Failed to set notes for game ${gameId}:`, error);
      toast.error(`Failed to save notes: ${error}`);
      throw error;
    }
  }
}

export const gamesStore = new GamesStore();
