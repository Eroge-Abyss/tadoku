import { gamesStore } from '$lib/stores/games.svelte';
import type { CurrentGame } from '$lib/types';

class SessionStore {
  #currentGame: CurrentGame | null = $state(null);
  #currentPlaytime: number = $state(0);
  #refreshInterval: ReturnType<typeof setInterval> | undefined;
  #contextMenu = $state({
    visible: false,
    x: 0,
    y: 0,
    gameId: null as string | null,
  });

  get currentGame() {
    return this.#currentGame;
  }

  get currentPlaytime() {
    return this.#currentPlaytime;
  }

  get contextMenu() {
    return this.#contextMenu;
  }

  showContextMenu(x: number, y: number, gameId: string) {
    this.#contextMenu = { visible: true, x, y, gameId };
  }

  hideContextMenu() {
    this.#contextMenu.visible = false;
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

  destroy(): void {}
}

export const sessionStore = new SessionStore();
