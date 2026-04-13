import { toast } from 'svelte-sonner';
import { gamesStore } from '$lib/stores/games.svelte';
import type { Novel } from '$lib/types';

export function useNovelNotes(getNovel: () => Novel) {
  let notes = $state('');
  let originalNotes = $state('');
  let editingNotes = $state(false);

  $effect(() => {
    const novel = getNovel();
    if (novel && !editingNotes) {
      notes = novel.notes ?? '';
      originalNotes = novel.notes ?? '';
    }
  });

  async function handleSaveNotes() {
    const novel = getNovel();
    try {
      await gamesStore.setGameNotes(novel.id, notes);
      toast.success('Notes saved successfully');
      originalNotes = notes;
      editingNotes = false;
    } catch {
      // Error is handled in gamesStore
    }
  }

  function handleCancelEdit() {
    editingNotes = false;
    notes = originalNotes;
  }

  function handleStartEdit() {
    editingNotes = true;
  }

  return {
    get notes() {
      return notes;
    },
    set notes(v) {
      notes = v;
    },
    get editingNotes() {
      return editingNotes;
    },
    handleSaveNotes,
    handleCancelEdit,
    handleStartEdit,
  };
}
