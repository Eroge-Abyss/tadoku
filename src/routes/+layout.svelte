<script>
  import '../app.css';
  import Sidebar from "$lib/Sidebar.svelte"; 
  import { getCurrentWindow } from '@tauri-apps/api/window';

  // when using `"withGlobalTauri": true`, you may use
  // const { getCurrentWindow } = window.__TAURI__.window;

  const appWindow = getCurrentWindow();

  document.getElementById('titlebar-minimize')
    ?.addEventListener('click', () => appWindow.minimize());
  document.getElementById('titlebar-close')
    ?.addEventListener('click', () => appWindow.close());
</script>
<main>
  <Sidebar />
  <slot />
</main>

<style>
  main {
    display: grid;
    grid-template-columns: 100px 1fr; 
  }
</style>
