<script>
  import '@fontsource-variable/noto-sans-jp';
  import '@fortawesome/fontawesome-free/css/all.min.css';
  import '../app.css';
  import Sidebar from '$lib/Sidebar.svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { appState } from './state.svelte';
  import { check } from '@tauri-apps/plugin-updater';
  import UpdateDialog from '$lib/util/UpdateDialog.svelte';

  // when using `"withGlobalTauri": true`, you may use
  // const { getCurrentWindow } = window.__TAURI__.window;

  let { children } = $props();
  let showUpdateDialog = $state(false);
  let updateInfo = $state(null);

  const appWindow = getCurrentWindow();

  onMount(() => {
    // Check for updates
    check()
      .then(async (update) => {
        if (!update) {
          return;
        }

        // Update the info and show dialog
        updateInfo = {
          version: update.version,
          notes: update.body,
        };
        showUpdateDialog = true;
      })
      .catch(console.error);
  });

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
      console.log(e.payload);

      appState.currentGame = e.payload;
    });

    listen('playtime', (e) => console.log(e.payload));
  });
</script>

<main>
  {@render children()}
  <Sidebar />

  {#if updateInfo}
    <UpdateDialog bind:isOpen={showUpdateDialog} {updateInfo} />
  {/if}
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
