<script>
    import Icon from "@iconify/svelte";
    import { open } from "@tauri-apps/plugin-dialog";
    import { invoke } from "@tauri-apps/api/core";
    import { appState } from "../routes/state.svelte";

    let showModal = $state(false);
    let search = $state();
    let exe_path = $state();
    let results = $state.raw([]);
    let selectedVn = $state.raw();

    $effect(async () => {
        const data = await invoke("fetch_vn_info", { key: search });
        results = search ? data : [];
        console.log(results);
    });

    const openModal = () => (showModal = true);
    const closeModal = () => {
        showModal = false;
        results = [];
        search = "";
        selectedVn = "";
    };

    const pickFile = async () => {
        const file = await open({
            multiple: false,
            directory: false,
            filters: [
                {
                    name: "Game exe or shortcut path",
                    extensions: ["exe", "lnk"],
                },
            ],
        });
        console.log(file);
        exe_path = file;
    };

    const selectGame = (game) => {
        selectedVn = game;
        results = [];
        search = "";
        console.log(selectedVn);
    };

    const saveGame = (vn) => {
        invoke("save_game", {
            gameId: vn.id,
            game: {
                title: vn.title,
                description: vn.description,
                image_url: vn.image.url,
                exe_file_path: exe_path,
                playtime: 0,
                is_pinned: false
            },
        })
            .then(async () => {
                appState.gamesList = await invoke("load_games");
                closeModal();
            })
            .catch(console.log);
    };
</script>

<section>
    <button id="btn__add" onclick={openModal}> + </button>

    <section id="modal" class:open={showModal}>
        <section class="modal__content">
            <header>
                <h3>Add a game</h3>
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <span onclick={closeModal}>
                    <Icon
                        icon="material-symbols-light:close-rounded"
                        style="font-size: 24px;"
                    />
                </span>
            </header>
            <section class="game-form">
                <div class="form-group">
                    <input bind:value={search} placeholder="Name or ID" />
                </div>

                <div id="suggestions">
                    {#each results as vn}
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <div
                            class="suggestion-item"
                            onclick={() => selectGame(vn)}
                        >
                            <div class="suggestion-image">
                                <img src={vn?.image?.url} alt={vn?.title} />
                            </div>
                            <div class="suggestion-text">
                                <p class="suggestion-title">{vn?.title}</p>
                                <p class="suggestion-id">{vn?.id}</p>
                            </div>
                        </div>
                    {/each}
                </div>

                {#if selectedVn}
                    <div class="selected-suggestion">
                        <img
                            src={selectedVn.image.url}
                            alt={selectedVn.title}
                        />
                        <div class="suggestion-text">
                            <p class="selected-suggestion-title">
                                {selectedVn.title}
                            </p>
                            <p class="selected-suggestion-id">
                                {selectedVn.id}
                            </p>
                        </div>
                    </div>
                {/if}

                <button onclick={pickFile}>Pick exe</button>
                <button
                    style="background: #9ece6a"
                    onclick={() => saveGame(selectedVn)}>Save</button
                >
            </section>
        </section>
    </section>
</section>

<style>
    #btn__add {
        border: 0;
        background: #313131;
        color: #5d5d5d;
        border: 2px solid #5d5d5d;
        height: 56px;
        width: 56px;
        font-size: 2.5rem;
        text-align: center;
        margin: auto;
        border-radius: 5px;
        cursor: pointer;
    }

    #modal {
        position: fixed;
        height: 100%;
        width: 100%;
        top: 0;
        left: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        color: #fff;
        background: rgba(0, 0, 0, 0.6);
        opacity: 0;
        pointer-events: none;
        transition: all 0.2s ease-in-out;
        /* Start scaled down */
        &.open {
            opacity: 1;
            pointer-events: all;

            & .modal__content {
                transform: translate(0, 0) scale(1); /* Scale up */
            }
        }

        & .modal__content {
            background-color: var(--main-background);
            padding: 1rem;
            width: 50%;
            display: flex;
            flex-direction: column;
            transform: translate(0, 100%) scale(0.8);
            transition: all 0.2s ease-in-out;
            & header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                color: var(--main-text);
                span {
                    color: #444;
                    cursor: pointer;
                    transition: color 0.2s ease-in-out;
                    &:hover {
                        color: var(--main-text);
                    }
                }
            }

            & .game-form {
                margin: 1rem;
                & .form-group {
                    display: grid;
                    grid-template-columns: repeat(2, 1fr);
                    gap: 1rem;
                    & input {
                        background-color: #313131;
                        border: 0;
                        padding: 0.5rem;
                        color: var(--main-text);
                        max-width: 400px;
                    }
                }

                & button {
                    border: 0;
                    background-color: #313131;
                    color: #fff;
                    width: 100%;
                    padding: 0.5rem;
                    font-size: 18px;
                    margin-top: 1rem;
                    cursor: pointer;
                }
            }
        }
    }

    #suggestions {
        margin-top: 10px;
        max-width: 400px;
        max-height: 200px;
        overflow: scroll;
    }

    /* Suggestion Item Styling */
    .suggestion-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 10px;
        background-color: #1b1b1b;
        border-radius: 4px;
        margin-bottom: 5px;
        cursor: pointer;
        transition: background-color 0.3s ease;
        color: var(--main-text);
    }

    .suggestion-item:hover {
        background-color: #bb9af7;

        & .suggestion-id {
            color: #cfc9c2;
        }
    }

    /* Suggestion Text Styling */
    .suggestion-text {
        flex: 1;
        padding: 1rem;
    }

    .suggestion-title {
        font-size: 16px;
    }

    .suggestion-id {
        font-size: 14px;
        color: #aaa;
    }

    /* Suggestion Image Styling */
    .suggestion-image img,
    .selected-suggestion img {
        width: 60px;
        height: 60px;
        border-radius: 4px;
        object-fit: cover;
    }

    .selected-suggestion {
        margin-top: 20px;
        padding: 10px;
        background-color: #1b1b1b;
        border-radius: 4px;
        color: #fff;
        display: flex;
        align-items: center;
    }

    .selected-suggestion img {
        width: 100px;
        height: 100px;
        border-radius: 4px;
        margin-top: 10px;
    }

    .selected-suggestion-title {
        font-size: 16px;
        color: var(--main-text);
    }

    .selected-suggestion-id {
        color: #aaa;
        font-size: 14px;
    }
</style>
