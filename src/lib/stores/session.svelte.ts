import { gamesStore } from '$lib/stores/games.svelte';
import type { CurrentGame } from '$lib/types';

class SessionStore {
  #currentGame: CurrentGame | null = $state(null);
  #currentPlaytime: number = $state(0);

  get currentGame() {
    return this.#currentGame;
  }

  get currentPlaytime() {
    return this.#currentPlaytime;
  }

  set(game: CurrentGame | null): void {
    this.#currentGame = game;
    this.#currentPlaytime = 0;
    gamesStore.refresh();
  }

  setCurrentPlaytime(playtime: number): void {
    this.#currentPlaytime = Math.max(0, Math.floor(playtime));
  }

  destroy(): void {}
}

export const sessionStore = new SessionStore();
