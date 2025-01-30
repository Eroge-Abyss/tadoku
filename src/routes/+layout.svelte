<script>
    import "../app.css";
    import Sidebar from "$lib/Sidebar.svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    // when using `"withGlobalTauri": true`, you may use
    // const { getCurrentWindow } = window.__TAURI__.window;

    const appWindow = getCurrentWindow();

    document
        .getElementById("titlebar-minimize")
        ?.addEventListener("click", () => appWindow.minimize());
    document
        .getElementById("titlebar-maximize")
        ?.addEventListener("click", () => appWindow.toggleMaximize());
    document
        .getElementById("titlebar-close")
        ?.addEventListener("click", () => appWindow.close());
</script>

<Sidebar />
<slot />
