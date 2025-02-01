<script>
    import { open } from "@tauri-apps/plugin-dialog";
    import { invoke } from "@tauri-apps/api/core";
    import { appState } from "../routes/state.svelte";
    import CloseIcon from "$lib/util/CloseIcon.svelte";
    const NSFW_RATE = 0.5;

    let showModal = $state(false);
    let search = $state();
    let exe_path = $state();
    let results = $state.raw([]);
    let selectedVn = $state.raw();
    let showImage = $state(false);

    function toggleImage() {
        showImage = !showImage;
    }

    async function updateSearch(e) {
        search = e.target.value;
        const data = await invoke("fetch_vn_info", { key: search });
        results = search ? data : [];
    }

    const openModal = () => (showModal = true);
    const closeModal = () => {
        showModal = false;
        results = [];
        search = "";
        selectedVn = "";
    };

    const debounce = (v) => {
        let timer;
        clearTimeout(timer);
        timer = setTimeout(() => {
            updateSearch(v); // TODO: make this function generic
        }, 750);
    };

    const pickFile = async () => {
        const file = await open({
            multiple: false,
            directory: false,
            filters: [
                {
                    name: "Game exe or shortcut path",
                    extensions: ["exe", "lnk", "bat"],
                },
            ],
        });
        exe_path = file;
    };

    const selectGame = (game) => {
        selectedVn = game;
        showImage = false;
        results = [];
        search = "";
    };

    const saveGame = async (vn) => {
        await appState.saveGame(vn.id, {
            title: vn.title,
            description: vn.description || "No Description",
            exe_file_path: exe_path,
            icon_url: null,
            image_url: vn.image.url,
            is_pinned: false,
            is_nsfw: vn.image.sexual > NSFW_RATE,
            playtime: 0,
        });
        closeModal();
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
                    <CloseIcon style="font-size: 24px;" />
                </span>
            </header>
            <section class="game-form">
                <div class="form-group">
                    <input
                        value={search}
                        onkeyup={(e) => debounce(e)}
                        placeholder="Name or ID"
                    />
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
                                {#if vn?.image?.sexual < NSFW_RATE}
                                    <img src={vn?.image?.url} alt={vn?.title} />
                                {:else}
                                    NSFW
                                {/if}
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
                        {#if selectedVn.image.sexual < NSFW_RATE || showImage}
                            <img
                                src={selectedVn.image.url}
                                alt={selectedVn.title}
                            />
                        {:else}
                            <div class="nsfw-warning" onclick={toggleImage}>
                                NSFW, click to show
                            </div>
                        {/if}
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
        z-index: 2;
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
        overflow-y: scroll;
        overflow-x: hidden;
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

    .nsfw-warning {
        cursor: pointer;
        color: red;
        text-decoration: underline;
    }
</style>
