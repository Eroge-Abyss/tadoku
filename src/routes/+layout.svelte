<script lang="ts">
  import '@fontsource-variable/noto-sans-jp';
  import '@fortawesome/fontawesome-free/css/all.min.css';
  import '../app.css';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount, type Snippet } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import UpdateDialog from '$lib/components/UpdateDialog.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import { Toaster } from 'svelte-sonner';
  import { appState } from '$lib/state.svelte';
  import { type Event } from '@tauri-apps/api/event';
  import type { CurrentGame } from '$lib/types';

  const { children }: { children: Snippet } = $props();
  const appWindow = getCurrentWindow();
  let refreshInterval: ReturnType<typeof setInterval>;

  onMount(() => {
    document
      .getElementById('titlebar-minimize')
      ?.addEventListener('click', () => appWindow.minimize());
    document
      .getElementById('titlebar-maximize')
      ?.addEventListener('click', () => appWindow.toggleMaximize());
    document
      .getElementById('titlebar-close')
      ?.addEventListener('click', () => appWindow.close());

    listen('current_game', (e: Event<CurrentGame | null>) => {
      appState.currentGame = e.payload;
      clearInterval(refreshInterval);

      if (e.payload != null) {
        refreshInterval = setInterval(async () => {
          await appState.refreshGamesList();
        }, 60_000);
      }

      appState.refreshGamesList();
    });

    // listen('chars_read_updated', () => {
    //   appState.refreshGamesList();
    // });
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
  <UpdateDialog />
  <Toaster
    position="top-right"
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
