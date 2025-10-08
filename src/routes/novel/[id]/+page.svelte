<script>
  import { fade } from 'svelte/transition';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { open } from '@tauri-apps/plugin-dialog';
  import ConfirmDialog from '$lib/components/ConfirmDialog.svelte';
  import { platform } from '@tauri-apps/plugin-os';
  import { invoke } from '@tauri-apps/api/core';
  import NovelHeader from '$lib/components/novel/NovelHeader.svelte';
  import TabContainer from '$lib/components/novel/TabContainer.svelte';
  import { appState } from '$lib/state.svelte';
  import { debounce } from '$lib/util';
  import ChangeProcess from '$lib/components/novel/ChangeProcess.svelte';

  const novel = $derived(appState.loadGame(page.params.id));

  if (!novel) {
    throw goto('/');
  }

  // Should I use derived?
  // yes
  // oki uwu
  // Derived values
  const hoursPlayed = $derived(Math.floor(novel.playtime / 3600));
  const minutesPlayed = $derived(Math.floor((novel.playtime % 3600) / 60));
  const todayHoursPlayed = $derived(
    Math.floor((novel.today_playtime || 0) / 3600),
  );
  const todayMinutesPlayed = $derived(
    Math.floor(((novel.today_playtime || 0) % 3600) / 60),
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
  let processList = $state();
  let processDialog = $state(false);
  let deleteDialog = $state(false);
  let selectedTab = $state('progress');
  let notes = $state(novel.notes);
  let originalNotes = novel.notes;

  const TABS = $state.raw([
    {
      label: 'Progress Overview',
      id: 'progress',
      visible: true,
    },
    {
      label: 'Characters',
      id: 'characters',
      visible: novel?.characters,
    },
    {
      label: 'Notes',
      id: 'notes',
      visible: true,
    },
  ]);

  // Effects
  $effect(() => {
    if (appState.currentGame && appState.currentGame.id === novel.id) {
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
        await appState.updateGameProcessPath(novel.id, newPath);
      }
    } else {
      processList = await invoke('get_active_windows');
      processDialog = true;
    }
  };

  const openDeleteDialog = () => {
    deleteDialog = true;
  };

  // Game actions
  const gameActions = {
    startGame: async () => {
      appState.startGame(novel.id);
    },

    stopGame: async () => {
      appState.closeGame();
    },

    togglePin: async () => {
      appState.togglePinned(novel.id);
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
        await appState.updateExePath(novel.id, newPath);
      }
    },

    deleteGame: async () => {
      await appState.deleteGame(novel.id);
      goto('/');
    },

    setNotes: debounce(invoke.bind(null, 'set_game_notes'), 300),
  };

  // Notes handlers
  const handleSaveNotes = () => {
    gameActions.setNotes({
      gameId: novel.id,
      notes,
    });
    originalNotes = notes;
    editingNotes = false;
  };

  const handleCancelEdit = () => {
    editingNotes = false;
    notes = originalNotes;
  };

  const handleStartEdit = () => {
    editingNotes = true;
  };

  // @ts-ignore
  function handleMenuClick(e) {
    // Check if the click occurred directly on the modal backdrop
    if (e.target?.classList.contains('secondary-menu')) {
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
    />

    <ConfirmDialog
      bind:isOpen={deleteDialog}
      onConfirm={gameActions.deleteGame}
      message={`Are you sure you want to delete <b style="color: red">${novel.title}</b>?`}
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
    padding: 25px;
  }

  .content {
    border-radius: var(--big-radius);
    display: flex;
    flex-direction: column;
    width: 100%;
  }
</style>
