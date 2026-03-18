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
  import type { ProcessItem, Tab } from '$lib/types';
  import { getAvailable } from '$lib/util';
  import { useGameActions } from '$lib/composables/useGameActions.svelte';
  import { usePlaytimeStats } from '$lib/composables/usePlaytimeStats.svelte';
  import { useNovelNotes } from '$lib/composables/useNovelNotes.svelte';
  import { toast } from 'svelte-sonner';
  import { goto } from '$app/navigation';
  import ProcessChangerDialog from '$lib/components/novel/ProcessChangerDialog.svelte';

  if (!page.params.id) {
    throw goto(resolve('/'));
  }

  const novel = $derived(gamesStore.getById(page.params.id as string));

  // svelte-ignore state_referenced_locally
  if (!novel) {
    throw goto(resolve('/'));
  }

  // Derived values
  const stats = usePlaytimeStats(() => novel);

  // State variables
  let playing = $state(false);
  let activeMenu = $state(false);
  let processList = $state<ProcessItem[]>([]);
  let processDialog = $state(false);
  let isDeleteDialogOpen = $state(false);
  let resetStatsDialog = $state(false);
  let selectedTab = $state('progress');
  let downloadingCharacters = $state(false);

  const novelNotes = useNovelNotes(() => novel);

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
  const gameActions = useGameActions(() => novel);

  const handleDownloadCharacters = async () => {
    downloadingCharacters = true;
    await gameActions.downloadCharacters();
    downloadingCharacters = false;
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

    <ProcessChangerDialog
      bind:isOpen={processDialog}
      gameId={novel.id}
      {processList}
    />

    <TabContainer
      {novel}
      bind:selectedTab
      tabs={TABS}
      hoursPlayed={stats.hoursPlayed}
      minutesPlayed={stats.minutesPlayed}
      todayHoursPlayed={stats.todayHoursPlayed}
      todayMinutesPlayed={stats.todayMinutesPlayed}
      firstPlayedDate={stats.firstPlayedDate}
      lastPlayedDate={stats.lastPlayedDate}
      {jitenCharCount}
      charsRead={novel.chars_read || 0}
      bind:notes={novelNotes.notes}
      bind:editingNotes={novelNotes.editingNotes}
      onSaveNotes={novelNotes.handleSaveNotes}
      onCancelEdit={novelNotes.handleCancelEdit}
      onStartEdit={novelNotes.handleStartEdit}
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
