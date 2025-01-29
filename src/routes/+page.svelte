<script>
    import GamesList from "$lib/GamesList.svelte";
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import { appState } from "./state.svelte";
    let playtime = 0;

    listen("playtime", (e) => {
        playtime = e.payload;
    });

    onMount(async () => {
        appState.gamesList = await invoke("load_games");
        console.log(appState);
    });

    // const test = async () => {
    //     const data = await invoke("fetch_vn_info", { key: "chaos head" });
    //     const item = data[1];
    //     console.log(data);

    //     await invoke("save_game", {
    //         gameId: item.id,
    //         game: {
    //             title: item.title,
    //             description: item.description,
    //             image_url: item.image.url,
    //             exe_file_path: "/usr/bin/thunar",
    //             playtime: 0,
    //         },
    //     });

    //     const games = await invoke("load_games");
    //     const game = games[item.id];
    //     id = item.id;

    //     if (game) {
    //         imageUrl = convertFileSrc(game.image_url);
    //     }
    // };

    // const del = async () => {
    //     await invoke("delete_game", { gameId: "v3091" });
    // };

    // const open = async () => {
    //     await invoke("open_game", { gameId: "v3091" });
    // };
</script>

<main class="container">
    <!--button on:click={test}>test</button>
    <button on:click={del}>del</button>
    <button on:click={open}>open</button>
    <h1>{playtime}</h1>
    <img src={imageUrl} alt="t" /-->

    <GamesList gamesList={appState.gamesList} />
</main>

<style>
</style>
