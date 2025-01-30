<script>
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import { fly, fade } from "svelte/transition";
    import { goto } from "$app/navigation";
    import { appState } from "../../state.svelte";
    import { page } from "$app/state";

    // let characterProgress = $derived(
    //     (novel.progress.charactersRead / novel.progress.totalCharacters) * 100,
    // );

    const novel = $state(appState.loadGame(page.params.id));

    if (!novel) {
        throw new Error("FIXME");
    }

    // Should I use derived?
    // yes
    let hoursPlayed = $derived(Math.floor(novel.playtime / 3600));
    let minutesPlayed = $derived(Math.floor((novel.playtime % 3600) / 60));

    const startGame = async () => {
        appState.startGame(novel.id);
    };

    const togglePin = async () => {
        appState.togglePinned(novel.id);

        novel.is_pinned = !novel.is_pinned;
    };

    const deleteGame = async () => {
        appState.deleteGame(novel.id);
        goto("/");
    };
</script>

<div class="container">
    <div class="content" in:fade={{ duration: 300 }}>
        <div class="header">
            <div
                class="novel-info"
                in:fly={{ x: 50, duration: 500, delay: 300 }}
            >
                <img
                    src={convertFileSrc(novel.image_url)}
                    alt={novel.title}
                    class="novel-image"
                    in:fly={{ y: 50, duration: 500, delay: 300 }}
                />
                <div>
                    <h1>{novel.title}</h1>
                    <p class="description">{novel.description}</p>
                </div>
            </div>
            <div class="buttons">
                <button onclick={startGame}>Play</button>
                <button onclick={togglePin}
                    >{novel.is_pinned ? "Unpin" : "Pin"}</button
                >
                <button onclick={deleteGame}>Delete</button>
            </div>
        </div>

        <div
            class="progress-overview"
            in:fly={{ y: 50, duration: 500, delay: 600 }}
        >
            <h2>Progress Overview</h2>

            <div class="stats-grid">
                <div
                    class="stat-item"
                    in:fly={{ y: 20, duration: 300, delay: 900 }}
                >
                    <p class="stat-label">Time Played</p>
                    <span class="stat-value"
                        >{hoursPlayed}h {minutesPlayed}m</span
                    >
                </div>
                <!-- <div
                    class="stat-item"
                    in:fly={{ y: 20, duration: 300, delay: 1000 }}
                >
                    <span class="stat-label">Characters Read</span>
                    <span class="stat-value"
                        >{novel.progress.charactersRead.toLocaleString()}</span
                    >
                </div>
                <div
                    class="stat-item"
                    in:fly={{ y: 20, duration: 300, delay: 1100 }}
                >
                    <span class="stat-label">Characters Remaining</span>
                    <span class="stat-value"
                        >{(
                            novel.progress.totalCharacters -
                            novel.progress.charactersRead
                        ).toLocaleString()}</span
                    >
                </div> -->
            </div>

            <!--div class="progress-bars">
                <div
                    class="progress-item"
                    in:fly={{ x: -20, duration: 300, delay: 1200 }}
                >
                    <div class="progress-label">
                        <span>Overall Progress</span>
                        <span>{novel.progress.completion}%</span>
                    </div>
                    <ProgressBar progress={novel.progress.completion} />
                </div>

                <div
                    class="progress-item"
                    in:fly={{ x: -20, duration: 300, delay: 1300 }}
                >
                    <div class="progress-label">
                        <span>Characters</span>
                        <span>{Math.round(characterProgress)}%</span>
                    </div>
                    <ProgressBar
                        progress={characterProgress}
                        color="var(--primary-dark)"
                    />
                </div>
            </div-->
        </div>
    </div>
</div>

<style>
    /* TODO: FIX old CSS vars */
  

    /* .back-button {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        color: var(--primary);
        text-decoration: none;
        margin-bottom: 2rem;
        font-size: 1rem;
        font-weight: 500;
        transition: color 0.2s;
    }

    .back-button:hover {
        color: var(--primary-dark);
    } */

    .content {
        border-radius: 12px;
        display: flex;
        flex-direction: column;
    }

    .header {
        display: flex;
        align-items: center;
        justify-content: center;
        flex-direction: column;
        padding: 2rem;
        gap: 1rem;
        /*background: linear-gradient(
            to bottom,
            rgba(0, 0, 0, 0.7) 0%,
            rgba(0, 0, 0, 0) 100%
        );*/
    }

    .novel-info {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .novel-image {
        width: 150px;
        height: 200px;
        object-fit: cover;
        border-radius: 8px;
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
    }
    h1 {
        color: var(--foreground);
        margin: 0 0 0.5rem 0;
        font-size: 2.5rem;
        font-weight: 700;
    }

    .description {
        flex: 1;
        color: var(--main-text);
        font-size: 14px;
        font-weight: 600;
        line-height: 1.5;
        overflow: scroll;
        max-height: 200px;
    }

    .buttons {
        display: flex;
        gap: 1rem;
        margin-right: auto;
        & button {
            border: 0;
            background-color: #313131;
            color: var(--main-text);
            width: 200px;
            padding: 0.5rem;
            font-size: 18px;
            margin-top: 1rem;
            cursor: pointer;

            &:first-child {
                background: #9ece6a;
            }

            &:last-child {
                background: #f7768e;
            }
        }
    }

    .progress-overview {
        padding: 2rem;
        display: flex;
        flex-direction: column;
    }

    h2 {
        color: var(--foreground);
        margin-bottom: 1.5rem;
        font-size: 1.5rem;
        font-weight: 600;
    }

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .stat-item {
        background-color: var(--accent);
        padding: 1rem;
        border-radius: 8px;
        transition:
            transform 0.2s,
            box-shadow 0.2s;
    }

    .stat-item:hover {
        transform: translateY(-5px);
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
    }

    .stat-label {
        color: var(--foreground);
        font-size: 0.875rem;
        margin-bottom: 0.25rem;
        opacity: 0.7;
    }

    .stat-value {
        color: var(--foreground);
        font-size: 1.25rem;
        font-weight: 600;
    }

    .progress-bars {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .progress-item {
        background-color: var(--accent);
        padding: 1rem;
        border-radius: 8px;
    }

    .progress-label {
        display: flex;
        justify-content: space-between;
        color: var(--foreground);
        font-size: 0.875rem;
        margin-bottom: 0.5rem;
        opacity: 0.7;
    }
</style>
