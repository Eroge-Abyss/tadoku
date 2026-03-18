import { invoke } from '@tauri-apps/api/core';
import { toast } from 'svelte-sonner';
import type { VndbResult } from '$lib/types';

export function useVndbSearch() {
  let search = $state('');
  let results = $state.raw<VndbResult[]>([]);
  let selectedVn = $state<VndbResult | null>(null);

  async function updateSearch() {
    try {
      const data = await invoke<VndbResult[]>('fetch_vn_info', { key: search });
      results = search ? data : [];
    } catch (error) {
      console.error('Error fetching VN info:', error);
      toast.error(`Search failed: ${error}`);
    }
  }

  function selectGame(game: VndbResult) {
    selectedVn = game;
    results = [];
    search = '';
  }

  function reset() {
    search = '';
    results = [];
    selectedVn = null;
  }

  function searchFromPath(file: string | null) {
    if (!file || search || results.length || selectedVn) return;

    const pathParts = file.split(/[\\/]/) || [];
    if (pathParts.length >= 2) {
      const fileName = pathParts[pathParts.length - 1];
      const parentFolder = pathParts[pathParts.length - 2];
      const fileNameWithoutExt = fileName.includes('.')
        ? fileName.split('.').slice(0, -1).join('.')
        : fileName;
      search = `${parentFolder} ${fileNameWithoutExt}`;
    } else if (pathParts.length === 1) {
      const fileName = pathParts[0];
      search = fileName.includes('.')
        ? fileName.split('.').slice(0, -1).join('.')
        : fileName;
    }

    if (search) {
      updateSearch();
    }
  }

  return {
    get search() { return search; },
    set search(value) { search = value; },
    get results() { return results; },
    get selectedVn() { return selectedVn; },
    updateSearch,
    selectGame,
    reset,
    searchFromPath
  };
}
