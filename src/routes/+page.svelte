<script>
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    let imageUrl = "";
    let id = null;
    const test = async () => {
        const data = await invoke("fetch_vn_info", { key: "chaos head" });
        const item = data[1];
        console.log(data);

        await invoke("save_game", {
            gameId: item.id,
            game: {
                title: item.title,
                description: item.description,
                image_url: item.image.url,
                exe_file_path: "test",
                play_time: 0,
            },
        });

        const games = await invoke("load_games");
        const game = games[item.id];
        id = item.id;

        if (game) {
            imageUrl = convertFileSrc(game.image_url);
        }
    };

    const del = async () => {
        await invoke("delete_game", { gameId: "v3091" });
    };

    const open = async () => {
        await invoke("open_game", { exePath: "/usr/bin/waypaper" });
    };
</script>

<main class="container">
    <button on:click={test}>test</button>
    <button on:click={del}>del</button>
    <button on:click={open}>open</button>
    <img src={imageUrl} alt="t" />
</main>

<style>
</style>
