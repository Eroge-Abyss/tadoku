<script>
  import { convertFileSrc, invoke } from '@tauri-apps/api/core'
  import { fly, fade } from 'svelte/transition'
  import { goto } from '$app/navigation'
  import { appState } from '../../state.svelte'
  import { page } from '$app/state'
  import { open } from '@tauri-apps/plugin-dialog'
  import ConfirmDialog from '$lib/util/confirmDialog.svelte'
  import { listen } from '@tauri-apps/api/event'
  // let characterProgress = $derived(
  //     (novel.progress.charactersRead / novel.progress.totalCharacters) * 100,
  // );

  let showImage = $state(false)
  function toggleImage() {
    showImage = !showImage
  }

  const novel = $state(appState.loadGame(page.params.id))
  let playing = $state(false)
  let activeMenu = $state(false)
  let deleteDialog = $state(false)
  const openDeleteDialog = () => {
    deleteDialog = true
  }
  $effect(() => {
    console.log('delete', deleteDialog)
  })

  if (!novel) {
    throw new Error('FIXME')
  }

  $effect(() => {
    if (appState.currentGame && appState.currentGame.id == novel.id) {
      playing = true
    } else {
      playing = false
    }
  })

  // Should I use derived?
  // yes
  // oki uwu
  let hoursPlayed = $derived(Math.floor(novel.playtime / 3600))
  let minutesPlayed = $derived(Math.floor((novel.playtime % 3600) / 60))

  const startGame = async () => {
    appState.startGame(novel.id)
  }

  const stopGame = async () => {
    appState.closeGame()
  }

  const togglePin = async () => {
    appState.togglePinned(novel.id)

    novel.is_pinned = !novel.is_pinned
  }

  const editExe = async () => {
    const newPath = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: 'Game exe or shortcut path',
          extensions: ['exe', 'lnk', 'bat'],
        },
      ],
    })

    if (newPath) {
      await appState.updateExePath(novel.id, newPath)
    }
  }

  const deleteGame = async () => {
    appState.deleteGame(novel.id)
    console.log('game deleted')
    goto('/')
  }
</script>

<div class="container">
  <div class="content" in:fade={{ duration: 100 }}>
    <div class="header">
      <div class="novel-info" in:fly={{ x: 10, duration: 500 }}>
        {#if !novel.is_nsfw || showImage}
          <img
            src={convertFileSrc(novel.image_url)}
            alt={novel.title}
            class="novel-image"
            in:fly={{ y: 50, duration: 500, delay: 300 }}
          />
        {:else}
          <img
            src={convertFileSrc(novel.image_url)}
            alt={novel.title}
            class="novel-image blur"
            in:fly={{ y: 50, duration: 500, delay: 300 }}
          />
        {/if}
        <div class="novel-text">
          <h1>{novel.title}</h1>
          <p class="description">{novel.description}</p>
        </div>
      </div>
      <div class="buttons">
        {#if playing}
          <button onclick={stopGame} class="playing">Close</button>
        {:else}
          <button onclick={startGame}>Start</button>
        {/if}
        <i
          class="fa-solid fa-ellipsis fa-xl"
          onclick={() => (activeMenu = !activeMenu)}
          class:active={activeMenu}
        ></i>
        <div class="menu" class:active={activeMenu}>
          <i
            onclick={togglePin}
            class={[
              'fa-solid',
              novel.is_pinned ? 'fa-thumbtack-slash' : 'fa-thumbtack',
            ]}
            title="Toggle pinned"
          ></i>
          <i
            onclick={editExe}
            class="fa-regular fa-pen-to-square"
            title="Edit exe path"
          ></i>
          <i
            onclick={openDeleteDialog}
            class="fa-regular fa-trash-can"
            style="color:  #f7768e;"
            title="Delete game"
          ></i>
        </div>
      </div>
    </div>
    <ConfirmDialog
      bind:isOpen={deleteDialog}
      onConfirm={deleteGame}
      message={`Are you sure you want to delete <b style="color: red">${novel.title}?</b>`}
    />
    <div
      class="progress-overview"
      in:fly={{ y: 50, duration: 500, delay: 600 }}
    >
      <h2>Progress Overview</h2>

      <div class="stats-grid">
        <div class="stat-item" in:fly={{ y: 20, duration: 300, delay: 900 }}>
          <p class="stat-label">Time Played</p>
          <span class="stat-value">{hoursPlayed}h {minutesPlayed}m</span>
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

  .blur {
    filter: blur(5px);
    transition: filter 0.2s ease-in-out;
  }

  .blur:hover {
    filter: blur(0);
  }

  .container {
    padding: 25px;
  }

  .content {
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    width: 100%;
  }

  .header {
    display: flex;
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

  .novel-text {
    height: 100%;
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
    overflow-x: hidden;
    overflow-y: scroll;
    max-height: 200px;
  }

  .buttons,
  .buttons .menu {
    display: flex;
    gap: 1rem;
    align-items: center;
    margin-top: 1rem;

    & .menu {
      margin-top: 0;
      opacity: 0;
      transform: translateX(-20px);
      transition:
        opacity 0.3s ease,
        transform 0.3s ease;
      pointer-events: none;
      & i {
        opacity: 0;
        transform: translateY(-10px);
        transition:
          opacity 0.3s ease,
          transform 0.3s ease;
        &:hover {
          color: var(--text-main);
        }
      }
      &.active {
        transform: translateX(0);
        z-index: 0;
        opacity: 1;
        & i {
          opacity: 1;
          transform: translateY(0);
          pointer-events: auto;
          &:nth-child(1) {
            transition-delay: 0.1s;
          }
          &:nth-child(2) {
            transition-delay: 0.2s;
          }
          &:nth-child(3) {
            transition-delay: 0.3s;
          }
        }
      }
    }
    i {
      color: #464545;
      transition: all 0.2s ease-in-out;
      cursor: pointer;
      &.active {
        color: var(--main-text);
        transform: rotate(90deg);
      }
    }

    & button {
      border: 0;
      background-color: #313131;
      color: var(--main-text);
      width: 200px;
      padding: 0.5rem;
      font-size: 18px;
      cursor: pointer;
      &:first-child {
        background: #9ece6a;

        &.playing {
          background: var(--main-mauve);
        }
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
    /* transition:
            transform 0.2s,
            box-shadow 0.2s; */
  }

  .stat-item:hover {
    /* transform: translateY(-5px); */
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
