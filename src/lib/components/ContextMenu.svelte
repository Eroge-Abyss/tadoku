<script lang="ts">
  import { sessionStore } from '$lib/stores/session.svelte';
  import { gamesStore } from '$lib/stores/games.svelte';
  import { useGameActions } from '$lib/composables/useGameActions.svelte';
  import { fly } from 'svelte/transition';
  import StatusSelector from './StatusSelector.svelte';
  import ConfirmDialog from './ConfirmDialog.svelte';
  import { onMount } from 'svelte';
  import { getPreferredTitle } from '$lib/util';

  // svelte-ignore non_reactive_update
  let menuRef: HTMLDivElement;
  let showStatusSubmenu = $state(false);
  let isDeleteDialogOpen = $state(false);

  const gameId = $derived(sessionStore.contextMenu.gameId);
  const game = $derived(gameId ? gamesStore.getById(gameId) : undefined);

  const actions = useGameActions(() => game);

  function close() {
    sessionStore.hideContextMenu();
    showStatusSubmenu = false;
  }

  async function toggleStatus(status: string) {
    if (!gameId) return;
    const currentStatuses = game?.categories || [];
    const newStatuses = currentStatuses.includes(status)
      ? currentStatuses.filter((s) => s !== status)
      : [...currentStatuses, status];
    await gamesStore.setGameCategories(gameId, newStatuses);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') close();
  }

  onMount(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (menuRef && !menuRef.contains(e.target as Node)) {
        close();
      }
    };
    window.addEventListener('mousedown', handleClickOutside);
    return () => window.removeEventListener('mousedown', handleClickOutside);
  });
</script>

<svelte:window onkeydown={handleKeydown} onblur={close} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
{#if sessionStore.contextMenu.visible && game}
  <div
    bind:this={menuRef}
    class="context-menu"
    role="menu"
    tabindex="-1"
    style="top: {sessionStore.contextMenu.y}px; left: {sessionStore.contextMenu
      .x}px;"
    transition:fly={{ duration: 100, y: 5 }}
    oncontextmenu={(e) => e.preventDefault()}
  >
    <div class="menu-header">
      <span class="game-title">{getPreferredTitle(game)}</span>
    </div>

    <div class="menu-divider"></div>

    <button
      class="menu-item"
      onclick={async () => {
        await actions.startGame();
        close();
      }}
    >
      <i class="fa-solid fa-play"></i>
      Start Game
    </button>

    <button
      class="menu-item"
      onclick={async () => {
        await actions.togglePin();
        close();
      }}
    >
      <i
        class={game.is_pinned
          ? 'fa-solid fa-thumbtack-slash'
          : 'fa-solid fa-thumbtack'}
      ></i>
      {game.is_pinned ? 'Unpin' : 'Pin'}
    </button>

    <div class="menu-item-with-submenu">
      <button
        class="menu-item"
        onmouseenter={() => (showStatusSubmenu = true)}
        onclick={() => (showStatusSubmenu = !showStatusSubmenu)}
      >
        <i class="fa-solid fa-tags"></i>
        Status
        <i class="fa-solid fa-chevron-right chevron"></i>
      </button>

      {#if showStatusSubmenu}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="status-submenu"
          role="menu"
          tabindex="-1"
          in:fly={{ x: 5, duration: 150 }}
          onmouseleave={() => (showStatusSubmenu = false)}
        >
          <StatusSelector
            categories={game.categories}
            {toggleStatus}
            clearStatuses={async () => {
              if (gameId) await gamesStore.setGameCategories(gameId, []);
            }}
          />
        </div>
      {/if}
    </div>

    <div class="menu-divider"></div>

    <button
      class="menu-item"
      onclick={async () => {
        await actions.editExe();
        close();
      }}
    >
      <i class="fa-regular fa-pen-to-square"></i>
      Edit Executable
    </button>

    <button
      class="menu-item danger"
      onclick={async () => {
        isDeleteDialogOpen = true;
        close(); // Close the context menu immediately
      }}
    >
      <i class="fa-regular fa-trash-can"></i>
      Delete Game
    </button>
  </div>
{/if}

{#if game}
  <ConfirmDialog
    bind:isOpen={isDeleteDialogOpen}
    title="Delete Game"
    onConfirm={actions.deleteGame}
    isDanger
    message={`Are you sure you want to delete <i class="danger-highlight">${game.title}</i> ?`}
  />
{/if}

<style>
  .context-menu {
    position: fixed;
    z-index: 9999;
    background: var(--main-background);
    border: 1px solid var(--accent);
    border-radius: var(--small-radius);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    padding: 0.5rem;
    min-width: 200px;
    user-select: none;
  }

  .menu-header {
    padding: 0.5rem 0.75rem;
    max-width: 250px;
  }

  .game-title {
    font-size: 0.8rem;
    font-weight: 700;
    color: var(--secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: block;
  }

  .menu-divider {
    height: 1px;
    background: var(--accent);
    margin: 0.4rem 0;
  }

  .menu-item {
    width: 100%;
    border: 0;
    border-radius: var(--small-radius);
    color: var(--main-text);
    background: transparent;
    padding: 0.6rem 0.75rem;
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    text-align: left;
  }

  .menu-item:hover {
    background: var(--accent);
  }

  .menu-item i {
    font-size: 14px;
    width: 16px;
    text-align: center;
    opacity: 0.8;
  }

  .menu-item.danger {
    color: #f7768e;
  }

  .menu-item.danger:hover {
    background: rgba(247, 118, 142, 0.1);
  }

  .menu-item-with-submenu {
    position: relative;
  }

  .chevron {
    margin-left: auto;
    font-size: 10px !important;
  }

  .status-submenu {
    position: absolute;
    left: 100%;
    top: -5px;
    background: var(--main-background);
    border: 1px solid var(--accent);
    border-radius: var(--small-radius);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
    padding: 0.5rem;
    min-width: 180px;
    margin-left: 0.25rem;
  }
</style>
