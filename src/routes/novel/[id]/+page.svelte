<script lang="ts">
  import { fade } from 'svelte/transition';
  import { resolve } from '$app/paths';
  import { page } from '$app/state';
  import { open } from '@tauri-apps/plugin-dialog';
  import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
  import { platform } from '@tauri-apps/plugin-os';
  import NovelHeader from '$lib/components/novel/NovelHeader.svelte';
  import TabContainer from '$lib/components/novel/TabContainer.svelte';
  import { gamesStore } from '$lib/stores/games.svelte';
  import { sessionStore } from '$lib/stores/session.svelte';
  import ChangeProcess from '$lib/components/novel/ChangeProcess.svelte';
  import type { ProcessItem, Tab } from '$lib/types';
  import { getAvailable } from '$lib/util';
  import { toast } from 'svelte-sonner';
  import { goto } from '$app/navigation';

  if (!page.params.id) {
    throw goto(resolve('/'));
  }

  const novel = $derived(gamesStore.getById(page.params.id as string));

  // svelte-ignore state_referenced_locally
  if (!novel) {
    throw goto(resolve('/'));
  }

  // Derived values
  const hoursPlayed = $derived(Math.floor(novel.playtime / 3600));
  const minutesPlayed = $derived(Math.floor((novel.playtime % 3600) / 60));
  // Goofy solution until I think of something better, at least it works xd
  const todayDate =
    new Date().getFullYear() +
    '-' +
    (new Date().getMonth() + 1).toString().padStart(2, '0') +
    '-' +
    new Date().getDate().toString().padStart(2, '0');
  const todayHoursPlayed = $derived(
    todayDate === novel.last_play_date
      ? Math.floor((novel.today_playtime || 0) / 3600)
      : 0,
  );
  const todayMinutesPlayed = $derived(
    todayDate === novel.last_play_date
      ? Math.floor(((novel.today_playtime || 0) % 3600) / 60)
      : 0,
  );
  const lastPlayedDate = $derived(
    novel.last_played ? new Date(novel.last_played * 1000) : null,
  );
  const firstPlayedDate = $derived(
    novel.first_played ? new Date(novel.first_played * 1000) : null,
  );

  // State variables
  let playing = $state(false);
  let activeMenu = $state(false);
  let editingNotes = $state(false);
  let processList = $state<ProcessItem[]>([]);
  let processDialog = $state(false);
  let isDeleteDialogOpen = $state(false);
  let resetStatsDialog = $state(false);
  let selectedTab = $state('progress');
  let notes = $state('');
  let originalNotes = '';
  let downloadingCharacters = $state(false);

  $effect(() => {
    if (novel && !editingNotes) {
      notes = novel.notes ?? '';
      originalNotes = novel.notes ?? '';
    }
  });

  // Jiten character count is now pre-fetched at startup and stored in game data
  const jitenCharCount = $derived(getAvailable(novel.jiten_char_count));

  const TABS: Tab[] = $derived([
    {
      label: 'Progress Overview',
      id: 'progress',
      visible: true,
      disabled: false,
      loading: false,
    },
    {
      label: 'Characters',
      id: 'characters',
      visible: true,
      disabled: !novel?.characters,
      loading: downloadingCharacters,
    },
    {
      label: 'Notes',
      id: 'notes',
      visible: true,
      disabled: false,
      loading: false,
    },
  ]);

  // Effects
  $effect(() => {
    if (
      sessionStore.currentGame &&
      novel &&
      sessionStore.currentGame.id === novel.id
    ) {
      playing = true;
    } else {
      playing = false;
    }
  });

  // Dialog handlers
  const openProcessDialog = async () => {
    if (platform() === 'linux') {
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
        await gamesStore.updateGameProcessPath(novel.id, newPath);
        toast.success('Process path updated');
      }
    } else {
      processList = await gamesStore.getActiveWindows();
      processDialog = true;
    }
  };

  const openDeleteDialog = () => {
    isDeleteDialogOpen = true;
  };

  const openResetStatsDialog = () => {
    resetStatsDialog = true;
  };

  // Game actions
  const gameActions = {
    startGame: async () => {
      if (novel) gamesStore.startGame(novel.id);
    },

    stopGame: async () => {
      gamesStore.closeGame();
    },

    togglePin: async () => {
      if (!novel) return;
      const wasPinned = novel.is_pinned;
      await gamesStore.togglePinned(novel.id);
      toast.success(wasPinned ? 'Game unpinned' : 'Game pinned');
    },

    editExe: async () => {
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
        if (!novel) return;
        await gamesStore.updateExePath(novel.id, newPath);
        toast.success('Executable path updated');
      }
    },

    deleteGame: async () => {
      if (!novel) return;
      await gamesStore.deleteGame(novel.id);
      toast.success('Game deleted');
      goto(resolve('/'));
    },

    resetStats: async () => {
      if (!novel) return;
      await gamesStore.resetStats(novel.id);
      toast.success('Stats reset successfully');
    },
  };

  // Notes handlers
  const handleSaveNotes = async () => {
    try {
      await gamesStore.setGameNotes(novel.id, notes);
      toast.success('Notes saved successfully');
      originalNotes = notes;
      editingNotes = false;
    } catch {
      // Error is handled in gamesStore
    }
  };

  const handleCancelEdit = () => {
    editingNotes = false;
    notes = originalNotes;
  };

  const handleStartEdit = () => {
    editingNotes = true;
  };

  const handleDownloadCharacters = async () => {
    downloadingCharacters = true;
    try {
      await gamesStore.setCharacters(novel.id);
      toast.success('Characters downloaded successfully');
    } catch (error) {
      console.error('Failed to download characters:', error);
    } finally {
      downloadingCharacters = false;
    }
  };

  function handleMenuClick(e: MouseEvent) {
    // Check if the click occurred directly on the modal backdrop
    if ((e.target as HTMLElement)?.classList.contains('secondary-menu')) {
      activeMenu = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="container">
  <div
    class="content"
    in:fade={{ duration: 100 }}
    onmousedown={handleMenuClick}
  >
    <NovelHeader
      {novel}
      {playing}
      bind:activeMenu
      onStartGame={gameActions.startGame}
      onStopGame={gameActions.stopGame}
      onTogglePin={gameActions.togglePin}
      onEditExe={gameActions.editExe}
      onProcessDialog={openProcessDialog}
      onDeleteDialog={openDeleteDialog}
      onResetStats={openResetStatsDialog}
      onDownloadCharacters={handleDownloadCharacters}
    />

    <ConfirmDialog
      bind:isOpen={isDeleteDialogOpen}
      title="Delete Game"
      onConfirm={gameActions.deleteGame}
      isDanger
      message={`Are you sure you want to delete <i class="danger-highlight">${novel.title}</i> ?`}
    />

    <ConfirmDialog
      bind:isOpen={resetStatsDialog}
      onConfirm={gameActions.resetStats}
      title="Reset Stats"
      isDanger
      message={`Are you sure you want to reset stats for <i class="danger-highlight">${novel.title}</i> ?`}
    />

    <ChangeProcess
      bind:isOpen={processDialog}
      gameId={novel.id}
      {processList}
    />

    <TabContainer
      {novel}
      bind:selectedTab
      tabs={TABS}
      {hoursPlayed}
      {minutesPlayed}
      {todayHoursPlayed}
      {todayMinutesPlayed}
      {firstPlayedDate}
      {lastPlayedDate}
      {jitenCharCount}
      charsRead={novel.chars_read || 0}
      bind:notes
      bind:editingNotes
      onSaveNotes={handleSaveNotes}
      onCancelEdit={handleCancelEdit}
      onStartEdit={handleStartEdit}
    />
  </div>
</div>

<style>
  .container {
    padding: 20px;
    padding-top: 0px;
    height: 100%;
    overflow-y: auto;
    box-sizing: border-box;
  }

  .content {
    border-radius: var(--big-radius);
    display: flex;
    flex-direction: column;
    width: 100%;
    padding: 0 25px;
    box-sizing: border-box;
  }
</style>
