<script>
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import bbobHTML from '@bbob/html';
  import html5Preset from '@bbob/preset-html5';
  import { fly, fade } from 'svelte/transition';
  import { goto } from '$app/navigation';
  import { appState } from '../../state.svelte';
  import { page } from '$app/state';
  import { open } from '@tauri-apps/plugin-dialog';
  import ConfirmDialog from '$lib/util/confirmDialog.svelte';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import ChangeProcess from '$lib/util/ChangeProcess.svelte';
  import { debounce, formatNotes } from '$lib/util/utils';
  import { platform } from '@tauri-apps/plugin-os';

  const novel = $derived(appState.loadGame(page.params.id));
  const customHtml5Preset = html5Preset.extend((tags) => ({
    ...tags,
    url: (node, params) => {
      const tag = tags.url(node, params, {});
      // @ts-ignore
      const href = tag.attrs?.href.startsWith('/')
        ? `https://vndb.org${tag.attrs?.href}`
        : tag.attrs?.href;
      console.log(href);

      return {
        ...tag,
        attrs: {
          ...tag.attrs,
          target: '_blank',
          rel: 'noopener noreferer',
          href,
        },
      };
    },
  }));

  if (!novel) {
    throw goto('/');
  }

  // Should I use derived?
  // yes
  // oki uwu
  const hoursPlayed = $derived(Math.floor(novel.playtime / 3600));
  const minutesPlayed = $derived(Math.floor((novel.playtime % 3600) / 60));
  const lastPlayedDate = $derived(
    novel.last_played ? new Date(novel.last_played * 1000) : null,
  );
  const firstPlayedDate = $derived(
    novel.first_played ? new Date(novel.first_played * 1000) : null,
  );

  let playing = $state(false);
  let activeMenu = $state(false);
  let editingNotes = $state(false);

  const TABS = $state.raw([
    {
      label: 'Progress Overview',
      id: 'progress',
      visible: true,
    },
    {
      label: 'Characters',
      id: 'characters',
      visible: novel?.characters,
    },
    {
      label: 'Notes',
      id: 'notes',
      visible: true,
    },
  ]);

  let processList = $state();
  let processDialog = $state(false);
  const openProcessDialog = async () => {
    if (platform() === 'linux') {
      const newPath = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: 'Game exe or shortcut path',
            extensions: ['exe', 'lnk', 'bat', 'sh'],
          },
        ],
      });

      if (newPath) {
        await appState.updateGameProcessPath(novel.id, newPath);
      }
    } else {
      processList = await invoke('get_active_windows');
      processDialog = true;
    }
  };

  let deleteDialog = $state(false);
  const openDeleteDialog = () => {
    deleteDialog = true;
  };

  let selectedTab = $state(TABS[0].id);
  let notes = $state(novel.notes);
  let originalNotes = novel.notes;

  $effect(() => {
    if (appState.currentGame && appState.currentGame.id === novel.id) {
      playing = true;
    } else {
      playing = false;
    }
  });

  const gameActions = {
    startGame: async () => {
      appState.startGame(novel.id);
    },

    stopGame: async () => {
      appState.closeGame();
    },

    togglePin: async () => {
      appState.togglePinned(novel.id);
    },

    editExe: async () => {
      const newPath = await open({
        multiple: false,
        directory: false,
        filters: [
          {
            name: 'Game exe or shortcut path',
            extensions: ['exe', 'lnk', 'bat', 'sh'],
          },
        ],
      });

      if (newPath) {
        await appState.updateExePath(novel.id, newPath);
      }
    },

    deleteGame: async () => {
      await appState.deleteGame(novel.id);
      goto('/');
    },

    setNotes: debounce(invoke.bind(null, 'set_game_notes'), 300),
  };

  /**
   * @param {Date} date
   */
  function formatRelativeDate(date) {
    const now = new Date();

    // Get dates without time for comparison
    const dateDay = new Date(
      date.getFullYear(),
      date.getMonth(),
      date.getDate(),
    );
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);

    // Format time portion consistently
    const timeFormat = new Intl.DateTimeFormat('en-US', {
      hour: 'numeric',
      minute: '2-digit',
      hour12: true,
    });

    const time = timeFormat.format(date);

    // Check if the date is today, yesterday, or older
    if (dateDay.getTime() === today.getTime()) {
      return `Today at ${time}`;
    } else if (dateDay.getTime() === yesterday.getTime()) {
      return `Yesterday at ${time}`;
    } else if (now.getTime() - date.getTime() < 7 * 24 * 60 * 60 * 1000) {
      // Within the last week, show day name
      const dayName = new Intl.DateTimeFormat('en-US', {
        weekday: 'long',
      }).format(date);
      return `${dayName} at ${time}`;
    } else {
      // Older than a week, show full date
      const dateFormat = new Intl.DateTimeFormat('en-US', {
        month: 'short',
        day: 'numeric',
        year: 'numeric',
      }).format(date);

      return `${dateFormat} at ${time}`;
    }
  }
</script>

<div class="container">
  <div class="content" in:fade={{ duration: 100 }}>
    <div class="header">
      <div class="novel-info" in:fly={{ x: 10, duration: 500 }}>
        {#if !novel.is_nsfw}
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
          <p class="description">
            {@html bbobHTML(novel.description, customHtml5Preset())}
          </p>
        </div>
      </div>
      <div class="buttons">
        {#if playing}
          <button onclick={gameActions.stopGame} class="playing">Close</button>
        {:else}
          <button onclick={gameActions.startGame}>Start</button>
        {/if}
        <i
          class="fa-solid fa-ellipsis fa-xl"
          onclick={() => (activeMenu = !activeMenu)}
          class:active={activeMenu}
        ></i>
        <div class="menu" class:active={activeMenu}>
          <i
            onclick={gameActions.togglePin}
            class={[
              'fa-solid',
              novel.is_pinned ? 'fa-thumbtack-slash' : 'fa-thumbtack',
            ]}
            title="Toggle pinned"
          ></i>
          <i
            onclick={gameActions.editExe}
            class="fa-regular fa-pen-to-square"
            title="Edit exe path"
          ></i>

          <i
            onclick={openProcessDialog}
            class="fa-solid fa-folder-tree"
            title="Change game process path"
          ></i>
          <i
            onclick={() => openUrl(`https://vndb.org/${novel.id}`)}
            class="fa-solid fa-arrow-up-right-from-square"
            title="Open in VNDB"
          ></i>
          <i
            onclick={() => invoke('set_characters', { gameId: novel.id })}
            class="fa-solid fa-user-plus"
            title="Download characters list"
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
      onConfirm={gameActions.deleteGame}
      message={`Are you sure you want to delete <b style="color: red">${novel.title}</b>?`}
    />
    <ChangeProcess
      bind:isOpen={processDialog}
      gameId={novel.id}
      {processList}
    />

    <div class="tabs" in:fly={{ y: 50, duration: 500, delay: 600 }}>
      <div class="tab">
        {#each TABS as tab}
          {#if tab.visible}
            <button
              class={selectedTab == tab.id ? 'active' : ''}
              onclick={() => (selectedTab = tab.id)}>{tab.label}</button
            >
          {/if}
        {/each}
      </div>
      <div class="tab-content">
        {#if selectedTab == 'progress'}
          <div class="stats-grid">
            <div
              class="stat-item"
              style="grid-column: span 2;"
              in:fly={{ y: 20, duration: 500 }}
            >
              <p class="stat-label">Time Played</p>
              <span class="stat-value">{hoursPlayed}h {minutesPlayed}m</span>
            </div>
            <div class="stat-item" in:fly={{ y: 20, duration: 500 }}>
              <p class="stat-label">First Played</p>
              <span class="stat-value">
                {#if firstPlayedDate}
                  {formatRelativeDate(firstPlayedDate)}
                {:else}
                  Never Played
                {/if}
              </span>
            </div>
            <div class="stat-item" in:fly={{ y: 20, duration: 500 }}>
              <p class="stat-label">Last Played</p>
              <span class="stat-value">
                {#if lastPlayedDate}
                  {formatRelativeDate(lastPlayedDate)}
                {:else}
                  Never Played
                {/if}
              </span>
            </div>
          </div>
        {:else if selectedTab == 'characters'}
          <div class="characters" in:fly={{ y: 20, duration: 300 }}>
            {#each novel.characters as character}
              <div
                class="character-card"
                onclick={() => openUrl(`https://vndb.org/${character.id}`)}
              >
                {#if character.image_url}
                  <img
                    src={convertFileSrc(character.image_url)}
                    alt={character.id}
                  />
                {:else}
                  <p>No Image</p>
                {/if}
                <div class="character-content">
                  <p class="main">{character.og_name}</p>
                  <p class="sub">{character.en_name}</p>
                </div>
                <i class="fa-solid fa-arrow-up-right-from-square"></i>
              </div>
            {/each}
          </div>
        {:else if selectedTab == 'notes'}
          <div class="notes" in:fly={{ y: 20, duration: 300 }}>
            {#if editingNotes}
              <textarea class="notes-div" bind:value={notes}></textarea>
              <div class="notes-actions">
                <button
                  class="save-button"
                  onclick={() => {
                    gameActions.setNotes({
                      gameId: novel.id,
                      notes,
                    });
                    editingNotes = false;
                  }}
                  title="Save Notes"
                >
                  Save
                </button>
                <button
                  class="cancel-button"
                  onclick={() => {
                    editingNotes = false;
                    notes = originalNotes;
                  }}
                  title="Cancel Editing"
                >
                  Cancel
                </button>
              </div>
            {:else}
              <div class="notes-div">
                {@html formatNotes(notes)}
              </div>
              <div class="notes-actions">
                <button
                  class="edit-notes-button"
                  onclick={() => {
                    editingNotes = true;
                  }}
                  title="Edit Notes"
                >
                  Edit Notes
                </button>
              </div>
            {/if}
          </div>
        {/if}
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
    border-radius: var(--big-radius);
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
          color: var(--main-text);
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
          &:nth-child(4) {
            transition-delay: 0.4s;
          }
          &:nth-child(5) {
            transition-delay: 0.5s;
          }
          &:nth-child(6) {
            transition-delay: 0.6s;
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
      border-radius: var(--small-radius);
      color: var(--main-text);
      background-color: #313131;
      width: 200px;
      padding: 0.5rem;
      font-size: 18px;
      cursor: pointer;
      transition: background-color 0.3s ease;

      &:first-child {
        background: var(--primary);

        &:hover {
          background: var(
            --primary-dark,
            color-mix(in srgb, var(--primary), #000 10%)
          ); /* test */
        }

        &.playing {
          background: rgb(224, 90, 90);
          &:hover {
            background: color-mix(in srgb, var(--primary), rgb(244, 65, 98));
          }
        }
      }
    }
  }

  .tabs {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;

    & .tab {
      display: flex;
      gap: 1.5rem;
      justify-content: flex-start;
      align-items: center;
      & button {
        background: var(--main-background);
        color: var(--main-text);
        border: 0;
        padding: 1rem;
        font-size: 1.5rem;
        text-align: center;
        position: relative;
        cursor: pointer;

        &:hover::after,
        &.active::after {
          transform: scaleX(1);
          opacity: 1;
        }

        &:after {
          content: '';
          position: absolute;
          bottom: 0;
          left: 0;
          display: block;
          height: 5px;
          width: 100%;
          background-color: var(--secondary);
          transform: scaleX(0);
          transform-origin: left;
          transition:
            transform 0.3s ease,
            opacity 0.3s ease;
          opacity: 0;
          border-radius: 2px;
        }
      }
    }
  }

  .characters {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
    gap: 1rem;
    grid-auto-rows: 1fr;
    padding: 1rem;
    & .character-card {
      display: flex;
      align-items: center;
      gap: 1.5rem;
      cursor: pointer;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
      transition:
        box-shadow 0.3s ease,
        transform 0.3s ease;
      background: var(--accent);
      border-radius: 8px;
      & .character-content {
        padding: 1rem;
        text-align: justify;
        & p.main {
          color: var(--main-text);
        }

        & p.sub {
          color: var(--secondary-text);
          font-size: 14px;
        }
      }
      & img {
        height: 100px;
        width: 100px;
        object-fit: cover;
        object-position: top;
        border-radius: 8px 0 0 8px;
      }

      & > p {
        width: 100px;
        text-align: center;
      }

      & i {
        margin-left: auto;
        padding: 1.5rem;
        color: var(--secondary-text);
      }

      &:hover {
        transform: translateY(-5px);
        box-shadow: 0 12px 20px rgba(0, 0, 0, 0.4);
        & i {
          color: var(--main-text);
        }
      }
    }
  }

  /* h2 {
    color: var(--foreground);
    margin-bottom: 1.5rem;
    font-size: 1.5rem;
    font-weight: 600;
  } */

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(1fr, 2);
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
    font-size: 1.5rem;
    font-weight: 600;
  }

  .notes-div {
    width: 100%;
    height: 300px; /* Fixed height */
    resize: none; /* Disable resizing */
    padding: 10px;
    margin: 0; /* Reset default margins */
    background-color: var(--accent);
    color: var(--main-text);
    border: 1px solid var(--secondary); /* Using secondary for border */
    border-radius: var(--small-radius);
    font-size: 1.1rem;
    outline: none; /* Remove default focus outline */
    overflow-x: auto; /* Enable horizontal scrolling for div */
    overflow-y: auto; /* Enable vertical scrolling for div */
    white-space: pre-wrap; /* Preserve whitespace and allow wrapping */
    word-wrap: break-word; /* Break long words */
  }

  .notes-div:focus {
    border-color: var(--secondary);
    box-shadow: 0 0 0 2px rgba(187, 154, 247, 0.5); /* A subtle glow */
  }

  textarea.notes-div {
    font-family: inherit;
    display: block;
    box-sizing: border-box;
    line-height: inherit;
    white-space: pre-wrap; /* Allow wrapping in textarea */
    word-wrap: break-word; /* Break long words */
    overflow-wrap: break-word; /* Modern property for word breaking */
  }

  /* Button styles for save, cancel, and edit actions */
  .save-button,
  .cancel-button,
  .edit-notes-button {
    border: 0;
    border-radius: var(--small-radius);
    color: var(--main-text);
    padding: 0.5rem 1rem;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
  }

  .save-button {
    background: var(--primary);
    &:hover {
      background: var(
        --primary-dark,
        color-mix(in srgb, var(--primary), #000 10%)
      );
    }
  }

  .cancel-button {
    background: #555555;
    &:hover {
      background: #666666;
    }
  }

  .edit-notes-button {
    background: var(--secondary);
    &:hover {
      background: color-mix(in srgb, var(--secondary), #000 10%);
    }
  }

  .notes-actions {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
    justify-content: flex-start;
  }

  /* .progress-bars {
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
  } */
</style>
