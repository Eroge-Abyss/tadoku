<script>
    import GamesList from "$lib/GamesList.svelte";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import { appState } from "./state.svelte";
    import { invoke } from "@tauri-apps/api/core";
    let playtime = 0;

    listen("playtime", (e) => {
        playtime = e.payload;
    });
    
    invoke("get_windows")

    onMount(() => appState.loadGames());
</script>

<main class="container">
    <GamesList gamesList={appState.gamesList} />
</main>
