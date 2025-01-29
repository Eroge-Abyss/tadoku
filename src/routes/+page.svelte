<script>
    import GamesList from "$lib/GamesList.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import { appState } from "./state.svelte";
    let playtime = 0;

    listen("playtime", (e) => {
        playtime = e.payload;
    });

    onMount(async () => {
        appState.gamesList = await invoke("load_games");
    });
</script>

<main class="container">
    <GamesList gamesList={appState.gamesList} />
</main>
