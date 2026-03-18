import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMount } from 'svelte';

export function useWindowTitlebar() {
  const appWindow = getCurrentWindow();

  onMount(() => {
    const minimize = document.getElementById('titlebar-minimize');
    const maximize = document.getElementById('titlebar-maximize');
    const close = document.getElementById('titlebar-close');

    const handleMinimize = () => appWindow.minimize();
    const handleMaximize = () => appWindow.toggleMaximize();
    const handleClose = () => appWindow.close();

    minimize?.addEventListener('click', handleMinimize);
    maximize?.addEventListener('click', handleMaximize);
    close?.addEventListener('click', handleClose);

    return () => {
      minimize?.removeEventListener('click', handleMinimize);
      maximize?.removeEventListener('click', handleMaximize);
      close?.removeEventListener('click', handleClose);
    };
  });
}
