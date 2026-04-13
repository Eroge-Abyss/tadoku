import { gamesStore } from '$lib/stores/games.svelte';
import type { CurrentGame } from '$lib/types';

class SessionStore {
  #currentGame: CurrentGame | null = $state(null);
  #currentPlaytime: number = $state(0);
  #refreshInterval: ReturnType<typeof setInterval> | undefined;

  get currentGame() {
    return this.#currentGame;
  }

  get currentPlaytime() {
    return this.#currentPlaytime;
  }

  set(game: CurrentGame | null): void {
    this.#currentGame = game;
    this.#currentPlaytime = 0;
    clearInterval(this.#refreshInterval);
    if (game) {
      this.#refreshInterval = setInterval(() => gamesStore.refresh(), 60_000);
    }
    gamesStore.refresh();
  }

  setCurrentPlaytime(playtime: number): void {
    this.#currentPlaytime = Math.max(0, Math.floor(playtime));
  }

  destroy(): void {
    clearInterval(this.#refreshInterval);
  }
}

export const sessionStore = new SessionStore();
