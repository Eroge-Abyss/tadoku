<script>
  import '../app.css';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  // when using `"withGlobalTauri": true`, you may use
  // const { getCurrentWindow } = window.__TAURI__.window;

  const appWindow = getCurrentWindow();

  document.getElementById('titlebar-minimize')
    ?.addEventListener('click', () => appWindow.minimize());
  document.getElementById('titlebar-close')
    ?.addEventListener('click', () => appWindow.close());
</script>

<slot />
