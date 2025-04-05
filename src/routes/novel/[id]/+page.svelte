<script>
  import CloseIcon from '$lib/util/CloseIcon.svelte';
  import { convertFileSrc, invoke } from '@tauri-apps/api/core'
  import { fly, fade } from 'svelte/transition'
  import { goto } from '$app/navigation'
  import { appState } from '../../state.svelte'
  import { page } from '$app/state'
  import { open } from '@tauri-apps/plugin-dialog'
  import ConfirmDialog from '$lib/util/confirmDialog.svelte';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import ProcessDropdown from '$lib/util/ProcessDropdown.svelte';
  import ChangeProcess from '$lib/util/ChangeProcess.svelte';
  
  // Initialize state variables
  let exe_path = $state("");
  let showModal = $state(false);
  let showProcessSelector = $state(false);
  let processList = $state([]);

  const closeProcessSelector = () => {
    showProcessSelector = false;
  };

  let showImage = $state(false);
  function toggleImage() {
    showImage = !showImage;
  }

  let processDialog = $state(false);
  const openProcessDialog = async () => {
    processList = await invoke('get_active_windows');
    processDialog = true;
  };

  // Load novel data
  const novel = $state(appState.loadGame(page.params.id));
  let playing = $state(false);
  let activeMenu = $state(false);
  let deleteDialog = $state(false);
  
  const openDeleteDialog = () => {
    deleteDialog = true;
  };
  
  const tabs = $state.raw([
    {
      label: 'Progress Overview',
      id: 'progress',
      visible: true,
    },
    {
      label: 'Characters',
      id: 'chars',
      visible: novel?.characters && novel.characters.length > 0,
    },
  ]);

  let selectedTab = $state(tabs[0].id);

  if (!novel) {
    throw new Error('Novel not found');
  }

  $effect(() => {
    if (appState.currentGame && appState.currentGame.id == novel.id) {
      playing = true;
    } else {
      playing = false;
    }
  });

  let hoursPlayed = $derived(Math.floor(novel.playtime / 3600));
  let minutesPlayed = $derived(Math.floor((novel.playtime % 3600) / 60));

  const startGame = async () => {
    appState.startGame(novel.id);
  };

  const stopGame = async () => {
    appState.closeGame();
  };

  const togglePin = async () => {
    appState.togglePinned(novel.id);
    novel.is_pinned = !novel.is_pinned;
  };

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
    });

    if (newPath) {
      await appState.updateExePath(novel.id, newPath);
    }
  };

  const deleteGame = async () => {
    appState.deleteGame(novel.id);
    goto('/');
  };
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
            onclick={toggleImage}
          />
        {/if}
        <div class="novel-text">
          <h1>{novel.title}</h1>
          <p class="description">{novel.description || 'No description available.'}</p>
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
            class={novel.is_pinned ? 'fa-solid fa-thumbtack' : 'fa-regular fa-thumbtack'}
            title={novel.is_pinned ? "Unpin novel" : "Pin novel"}
          ></i>
          
          <i
            onclick={editExe}
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
            onclick={openDeleteDialog}
            class="fa-regular fa-trash-can"
            style="color: #f7768e;"
            title="Delete game"
          ></i>
        </div>
      </div>
    </div>
    
    <ConfirmDialog
      bind:isOpen={deleteDialog}
      onConfirm={deleteGame}
      message={`Are you sure you want to delete <b style="color: #f7768e">${novel.title}</b>?`}
    />
    
    <ChangeProcess
      bind:isOpen={processDialog}
      gameId={novel.id}
      {processList}
    />

    <div class="tabs" in:fly={{ y: 50, duration: 500, delay: 600 }}>
      <div class="tab">
        {#each tabs as tab}
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
            <div class="stat-item" in:fly={{ y: 20, duration: 500 }}>
              <p class="stat-label">Time Played</p>
              <span class="stat-value">{hoursPlayed}時{minutesPlayed}分</span>
            </div>
          </div>
        {:else if selectedTab == 'chars' && novel.characters && novel.characters.length > 0}
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
                  <div class="no-image">No Image</div>
                {/if}
                <div class="character-content">
                  <p class="main">{character.og_name}</p>
                  <p class="sub">{character.en_name}</p>
                </div>
                <i class="fa-solid fa-arrow-up-right-from-square"></i>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .blur {
    filter: blur(5px);
    transition: filter 0.2s ease-in-out;
    cursor: pointer;
  }

  .blur:hover {
    filter: blur(3px);
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
    background-color: var(--accent);
    border-radius: 12px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
    margin-bottom: 1rem;
  }

  .novel-info {
    display: flex;
    align-items: flex-start;
    gap: 2rem;
  }

  .novel-text {
    height: 100%;
    flex: 1;
  }

  .novel-image {
    width: 150px;
    height: 200px;
    object-fit: cover;
    border-radius: 8px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
    transition: transform 0.3s ease;
  }
  
  .novel-image:hover {
    transform: scale(1.05);
  }
  
  h1 {
    color: var(--main-text);
    margin: 0 0 1rem 0;
    font-size: 2.5rem;
    font-weight: 700;
    transition: color 0.3s ease;
  }

  .description {
    color: var(--secondary-text);
    font-size: 14px;
    line-height: 1.6;
    max-height: 200px;
    overflow-y: auto;
    padding-right: 10px;
    transition: color 0.3s ease;
  }

  .description::-webkit-scrollbar {
    width: 4px;
  }

  .description::-webkit-scrollbar-thumb {
    background: var(--main-mauve);
    border-radius: 2px;
  }

  .buttons {
    display: flex;
    gap: 1rem;
    align-items: center;
    margin-top: 1rem;
  }
  
  .buttons .menu {
    display: flex;
    gap: 1rem;
    align-items: center;
    margin-top: 0;
    opacity: 0;
    transform: translateX(-20px);
    transition: opacity 0.3s ease, transform 0.3s ease;
    pointer-events: none;
  }
  
  .buttons .menu i {
    opacity: 0;
    transform: translateY(-10px);
    transition: opacity 0.3s ease, transform 0.3s ease, color 0.2s ease;
  }
  
  .buttons .menu i:hover {
    color: var(--main-text);
  }
  
  .buttons .menu.active {
    transform: translateX(0);
    z-index: 1;
    opacity: 1;
    pointer-events: auto;
  }
  
  .buttons .menu.active i {
    opacity: 1;
    transform: translateY(0);
    cursor: pointer;
  }
  
  .buttons .menu.active i:nth-child(1) {
    transition-delay: 0.1s;
  }
  
  .buttons .menu.active i:nth-child(2) {
    transition-delay: 0.2s;
  }
  
  .buttons .menu.active i:nth-child(3) {
    transition-delay: 0.3s;
  }
  
  .buttons .menu.active i:nth-child(4) {
    transition-delay: 0.4s;
  }
  
  .buttons .menu.active i:nth-child(5) {
    transition-delay: 0.5s;
  }
  
  .buttons i {
    color: var(--secondary-text);
    transition: all 0.2s ease-in-out;
    cursor: pointer;
  }
  
  .buttons i.active {
    color: var(--main-text);
    transform: rotate(90deg);
  }

  .buttons button {
    border: 0;
    background-color: var(--accent);
    color: var(--main-text);
    width: 200px;
    padding: 0.75rem;
    font-size: 18px;
    cursor: pointer;
    border-radius: 8px;
    transition: all 0.3s ease;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .buttons button:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }
  
  .buttons button:first-child {
    background: #9ece6a;
    color: rgba(0, 0, 0, 0.8);
  }
  
  .buttons button:first-child:hover {
    background: #8abb5b;
  }

  .buttons button.playing {
    background: var(--main-mauve);
    color: white;
  }
  
  .buttons button.playing:hover {
    background: rgba(var(--main-mauve-rgb, 187, 154, 247), 0.8);
  }

  .tabs {
    padding: 1rem 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .tab {
    display: flex;
    gap: 1.5rem;
    justify-content: flex-start;
    align-items: center;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .tab button {
    background: transparent;
    color: var(--secondary-text);
    border: 0;
    padding: 1rem;
    font-size: 1.1rem;
    text-align: center;
    position: relative;
    cursor: pointer;
    transition: color 0.3s ease;
  }
  
  .tab button:hover {
    color: var(--main-text);
  }
  
  .tab button:hover::after, 
  .tab button.active::after {
    transform: scaleX(1);
    opacity: 1;
  }
  
  .tab button::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    display: block;
    height: 3px;
    width: 100%;
    background-color: var(--main-mauve);
    transform: scaleX(0);
    transform-origin: left;
    transition: transform 0.3s ease, opacity 0.3s ease;
    opacity: 0;
    border-radius: 2px;
  }
  
  .tab button.active {
    color: var(--main-text);
  }

  .tab-content {
    padding: 1rem;
  }

  .characters {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 1rem;
    padding: 1rem 0;
  }
  
  .character-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    cursor: pointer;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
    transition: all 0.3s ease;
    background: var(--accent);
    border-radius: 8px;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }
  
  .character-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 15px rgba(0, 0, 0, 0.3);
  }
  
  .character-card:hover i {
    color: var(--main-mauve);
  }

  .character-content {
    flex: 1;
    padding: 1rem;
  }
  
  .character-content p.main {
    color: var(--main-text);
    font-weight: 600;
    margin: 0 0 0.5rem 0;
  }

  .character-content p.sub {
    color: var(--secondary-text);
    font-size: 14px;
    margin: 0;
  }
  
  .character-card img {
    height: 80px;
    width: 80px;
    object-fit: cover;
    flex-shrink: 0;
  }
  
  .no-image {
    height: 80px;
    width: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(255, 255, 255, 0.1);
    color: var(--secondary-text);
    font-size: 14px;
    flex-shrink: 0;
  }

  .character-card i {
    margin-left: auto;
    padding: 1.5rem;
    color: var(--secondary-text);
    transition: color 0.3s ease;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1.5rem;
    padding: 1rem 0;
  }

  .stat-item {
    background-color: var(--accent);
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
    transition: all 0.3s ease;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }

  .stat-item:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 15px rgba(0, 0, 0, 0.3);
  }

  .stat-label {
    color: var(--secondary-text);
    font-size: 0.875rem;
    margin-bottom: 0.5rem;
  }

  .stat-value {
    color: var(--main-text);
    font-size: 1.5rem;
    font-weight: 600;
  }

  .process-selector {
    position: fixed;
    height: 100%;
    width: 100%;
    z-index: 100;
    top: 0;
    left: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(5px);
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.3s ease;
  }
  
  .process-selector.open {
    opacity: 1;
    pointer-events: all;
  }
  
  .process-selector-content {
    background-color: var(--main-background);
    border-radius: 8px;
    padding: 2rem;
    width: 80%;
    max-width: 600px;
    position: relative;
  }
  
  .process-selector-content span {
    position: absolute;
    top: 1rem;
    right: 1rem;
    cursor: pointer;
    color: var(--secondary-text);
    transition: color 0.2s ease;
  }
  
  .process-selector-content span:hover {
    color: var(--main-text);
  }

  @media (max-width: 768px) {
    .novel-info {
      flex-direction: column;
      align-items: center;
    }
    
    .novel-text {
      text-align: center;
    }
    
    h1 {
      font-size: 1.8rem;
    }
    
    .characters {
      grid-template-columns: 1fr;
    }
  }
</style>