<script>
  import '@fontsource-variable/noto-sans-jp';
  import '@fortawesome/fontawesome-free/css/all.min.css';
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import UpdateDialog from '$lib/components/UpdateDialog.svelte';
  import { appState } from '$lib/state.svelte';
  import { app } from '@tauri-apps/api';

  // when using `"withGlobalTauri": true`, you may use
  // const { getCurrentWindow } = window.__TAURI__.window;

  const { children } = $props();
  const appWindow = getCurrentWindow();

  document
    .getElementById('titlebar-minimize')
    ?.addEventListener('click', () => appWindow.minimize());
  document
    .getElementById('titlebar-maximize')
    ?.addEventListener('click', () => appWindow.toggleMaximize());
  document
    .getElementById('titlebar-close')
    ?.addEventListener('click', () => appWindow.close());

  onMount(() => {
    listen('current_game', (e) => {
      appState.currentGame = e.payload;
      appState.refreshGamesList();
    });

    listen('flushed_playtime', () => {
      appState.refreshGamesList();
    });

    //listen('playtime', (e) => console.log(e.payload));
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
