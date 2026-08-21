<script lang="ts">
  import '@fontsource-variable/noto-sans-jp';
  import '@fortawesome/fontawesome-free/css/all.min.css';
  import '../app.css';
  import { useWindowTitlebar } from '$lib/composables/useWindowTitlebar.svelte';
  import { onMount, onDestroy, type Snippet } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import UpdateDialog from '$lib/components/UpdateDialog.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import ContextMenu from '$lib/components/ContextMenu.svelte';
  import { Toaster, toast } from 'svelte-sonner';
  import { gamesStore } from '$lib/stores/games.svelte';
  import { settingsStore } from '$lib/stores/settings.svelte';
  import { sessionStore } from '$lib/stores/session.svelte';
  import { type Event } from '@tauri-apps/api/event';
  import type { CurrentGame } from '$lib/types';

  const { children }: { children: Snippet } = $props();

  useWindowTitlebar();

  function handleKeydown(event: KeyboardEvent) {
    if ((event.ctrlKey || event.metaKey) && event.key === 'f') {
      event.preventDefault();
      const searchInput = document.getElementById('game-search-input');
      if (searchInput) {
        searchInput.focus();
      }
    }
  }

  let unlistenCurrentGame: (() => void) | undefined;
  let unlistenStatsSynced: (() => void) | undefined;
  let unlistenGameNotDetected: (() => void) | undefined;

  onMount(async () => {
    window.addEventListener('keydown', handleKeydown);
    await Promise.all([settingsStore.init(), gamesStore.init()]);

    unlistenCurrentGame = await listen(
      'current_game',
      (e: Event<CurrentGame | null>) => {
        sessionStore.set(e.payload);
      },
    );

    unlistenStatsSynced = await listen('stats_synced', () => {
      gamesStore.refresh();
    });

    unlistenGameNotDetected = await listen(
      'game_not_detected',
      (e: Event<{ id: string; title: string }>) => {
        toast.error(`Game isn't detected: ${e.payload.title}`, {
          duration: Number.POSITIVE_INFINITY,
        });
      },
    );
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleKeydown);
    if (unlistenCurrentGame) unlistenCurrentGame();
    if (unlistenStatsSynced) unlistenStatsSynced();
    if (unlistenGameNotDetected) unlistenGameNotDetected();
    sessionStore.destroy();
  });
</script>

<main>
  <Sidebar />
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-button" id="titlebar-minimize">
      <img src="minimize.svg" alt="minimize" />
    </div>
    <div class="titlebar-button" id="titlebar-maximize">
      <img src="maximize.svg" alt="maximize" />
    </div>
    <div class="titlebar-button" id="titlebar-close">
      <img src="close.svg" alt="close" />
    </div>
  </div>
  <div class="content">
    {@render children()}
  </div>
  <ContextMenu />
  <UpdateDialog />
  <Toaster
    position="top-right"
    offset={{
      top: 30,
    }}
    visibleToasts={1}
    toastOptions={{
      style:
        'background: var(--accent); color: var(--main-text); border: 1px solid rgba(255, 255, 255, 0.1); font-family: "Noto Sans JP Variable", sans-serif;',
    }}
  />
</main>

<style>
  main {
    display: grid;
    grid-template-columns: 85px 1fr;
    grid-template-rows: 30px 1fr;
    grid-template-areas:
      'sidebar titlebar'
      'sidebar content';
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }

  :global(nav) {
    grid-area: sidebar;
  }

  .titlebar {
    grid-area: titlebar;
  }

  .content {
    grid-area: content;
    overflow: hidden;
    position: relative;
  }
</style>
