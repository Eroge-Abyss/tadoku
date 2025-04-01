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

  // when using `"withGlobalTauri": true`, you may use
  // const { getCurrentWindow } = window.__TAURI__.window;

  let { children } = $props();

  const appWindow = getCurrentWindow();

  check()
    .then(async (update) => {
      if (!update) {
        return;
      }

      console.log(
        `found update ${update.version} from ${update.date} with notes ${update.body}`,
      );
      let downloaded = 0;
      let contentLength = 0;
      // alternatively we could also call update.download() and update.install() separately
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength;
            console.log(
              `started downloading ${event.data.contentLength} bytes`,
            );
            break;
          case 'Progress':
            downloaded += event.data.chunkLength;
            console.log(`downloaded ${downloaded} from ${contentLength}`);
            break;
          case 'Finished':
            console.log('download finished');
            break;
        }
      });

      console.log('update installed');
    })
    .catch(console.log);

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
