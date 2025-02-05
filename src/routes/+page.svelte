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

    let games = null;

    onMount(async () => {
        appState.loadGames();
        games = await invoke("get_active_windows");
    });
</script>

<main class="container">
    {#each games as game}
        <p>{game.title}</p>
        <p>{game.exe_path}</p>
        <img src={game.icon} width={32} height={32}/>
    {/each}

    <GamesList gamesList={appState.gamesList} />
</main>
