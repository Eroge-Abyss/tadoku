import { gamesStore } from '$lib/stores/games.svelte';
import type { CurrentGame } from '$lib/types';

class SessionStore {
  #currentGame: CurrentGame | null = $state(null);
  #refreshInterval: ReturnType<typeof setInterval> | undefined;

  get currentGame() {
    return this.#currentGame;
  }

  set(game: CurrentGame | null): void {
    this.#currentGame = game;
    clearInterval(this.#refreshInterval);
    if (game) {
      this.#refreshInterval = setInterval(() => gamesStore.refresh(), 60_000);
    }
    gamesStore.refresh();
  }

  destroy(): void {
    clearInterval(this.#refreshInterval);
  }
}

export const sessionStore = new SessionStore();
