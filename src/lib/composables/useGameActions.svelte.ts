import { open } from '@tauri-apps/plugin-dialog';
import { goto } from '$app/navigation';
import { resolve } from '$app/paths';
import { toast } from 'svelte-sonner';
import { gamesStore } from '$lib/stores/games.svelte';
import type { Novel } from '$lib/types';

export function useGameActions(getNovel: () => Novel | undefined) {
  return {
    startGame: async () => {
      const novel = getNovel();
      if (novel) await gamesStore.startGame(novel.id);
    },

    stopGame: async () => {
      await gamesStore.closeGame();
    },

    togglePin: async () => {
      const novel = getNovel();
      if (!novel) return;
      const wasPinned = novel.is_pinned;
      await gamesStore.togglePinned(novel.id);
      toast.success(wasPinned ? 'Game unpinned' : 'Game pinned');
    },

    editExe: async () => {
      const novel = getNovel();
      if (!novel) return;
      const newPath = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: 'Game exe or shortcut path',
            extensions: ['exe', 'lnk', 'bat', 'sh'],
          },
        ],
      });

      if (newPath) {
        await gamesStore.updateExePath(novel.id, newPath);
        toast.success('Executable path updated');
      }
    },

    deleteGame: async () => {
      const novel = getNovel();
      if (!novel) return;
      await gamesStore.deleteGame(novel.id);
      toast.success('Game deleted');
      goto(resolve('/'));
    },

    resetStats: async () => {
      const novel = getNovel();
      if (!novel) return;
      await gamesStore.resetStats(novel.id);
      toast.success('Stats reset successfully');
    },

    downloadCharacters: async () => {
      const novel = getNovel();
      if (!novel) return;
      try {
        await gamesStore.setCharacters(novel.id);
        toast.success('Characters downloaded successfully');
      } catch (error) {
        // Error is handled by gamesStore (toast.error)
      }
    },
  };
}
