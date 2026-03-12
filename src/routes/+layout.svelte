<script lang="ts">
  import '@fontsource-variable/noto-sans-jp';
  import '@fortawesome/fontawesome-free/css/all.min.css';
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount, type Snippet } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import UpdateDialog from '$lib/components/UpdateDialog.svelte';
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
  {@render children()}
  <Sidebar />
  <UpdateDialog />
</main>

<style>
  main {
    display: grid;
    grid-template-columns: 100px 1fr;
    grid-template-areas: 'sidebar content';
  }

  :global(nav) {
    grid-area: sidebar;
  }
</style>
