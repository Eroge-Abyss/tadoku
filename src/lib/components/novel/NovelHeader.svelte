<script lang="ts">
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import bbobHTML from '@bbob/html';
  import html5Preset from '@bbob/preset-html5';
  import { fly } from 'svelte/transition';
  import { revealItemInDir, openUrl } from '@tauri-apps/plugin-opener';
  import type { Novel } from '$lib/types';
  import { appState } from '$lib/state.svelte';
  import StatusSelector from '../StatusSelector.svelte';

  async function toggleStatus(status: string) {
    const currentStatuses = novel.categories || [];
    const newStatuses = currentStatuses.includes(status)
      ? currentStatuses.filter((s) => s !== status)
      : [...currentStatuses, status];
    await appState.setGameCategories(novel.id, newStatuses);
  }

  let {
    novel,
    playing,
    activeMenu = $bindable(),
    onStartGame,
    onStopGame,
    onTogglePin,
    onEditExe,
    onProcessDialog,
    onDeleteDialog,
  }: { novel: Novel; [key: string]: any } = $props();

  let menuToggleRef: HTMLButtonElement;
  // svelte-ignore non_reactive_update
  let secondaryMenuRef: HTMLDivElement;
  // svelte-ignore non_reactive_update
  let statusMenuRef: HTMLDivElement;
  let showStatusMenu = $state(false);

  // Function to handle clicks outside the menu and submenu
  function handleClickOutside(event: any) {
    // Close primary menu if click is outside it
    if (
      activeMenu &&
      secondaryMenuRef &&
      !secondaryMenuRef.contains(event.target) &&
      menuToggleRef &&
      !menuToggleRef.contains(event.target)
    ) {
      activeMenu = false;
      showStatusMenu = false; // Also close status menu if primary closes
    }

    // Close status submenu if click is outside it AND the main secondary menu,
    // but only if the main menu is still active (implies click was *inside* main menu but outside status submenu)
    if (
      showStatusMenu &&
      statusMenuRef &&
      !statusMenuRef.contains(event.target)
    ) {
      // We need to ensure the click wasn't on the "Status" button itself
      const statusToggleButton = secondaryMenuRef.querySelector(
        '.menu-item-with-submenu > .menu-item',
      );
      if (statusToggleButton && !statusToggleButton.contains(event.target)) {
        showStatusMenu = false;
      }
    }
  }

  function withMenuClose<T extends (...args: any[]) => any>(fn: T): T {
    return ((...args: Parameters<T>): ReturnType<T> => {
      activeMenu = false;
      showStatusMenu = false; // Close status menu too
      return fn(...args);
    }) as T;
  }

  const customHtml5Preset = html5Preset.extend((tags) => ({
    ...tags,
    url: (node, params) => {
      const tag = tags.url(node, params, {});
      // @ts-ignore
      const href = tag.attrs?.href.startsWith('/')
        ? `https://vndb.org${tag.attrs?.href}`
        : tag.attrs?.href;

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
</script>

<svelte:window onclick={handleClickOutside} />

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
      {#if appState.useJpForTitleTime && novel.alt_title}
        <h1>{novel.alt_title}</h1>
        <p class="alt-title">{novel.title}</p>
      {:else}
        <h1>{novel.title}</h1>
      {/if}
      <p class="description">
        {@html bbobHTML(novel.description, customHtml5Preset())}
      </p>
    </div>
  </div>
  <div class="action-buttons">
    <div class="primary-actions">
      {#if playing}
        <button onclick={onStopGame} class="primary-button playing">
          <i class="fa-solid fa-stop"></i>
          Close Game
        </button>
      {:else}
        <button onclick={onStartGame} class="primary-button">
          <i class="fa-solid fa-play"></i>
          Start Game
        </button>
      {/if}

      <button
        onclick={onTogglePin}
        class="action-button"
        class:pinned={novel.is_pinned}
      >
        <i
          class={novel.is_pinned
            ? 'fa-solid fa-thumbtack-slash'
            : 'fa-solid fa-thumbtack'}
        ></i>
        {novel.is_pinned ? 'Unpin' : 'Pin'}
      </button>

      <div class="more-actions-container">
        <button
          onclick={() => (activeMenu = !activeMenu)}
          class="menu-toggle"
          class:active={activeMenu}
          bind:this={menuToggleRef}
        >
          <i class="fa-solid fa-ellipsis-vertical"></i>
          More
        </button>

        {#if activeMenu}
          <div
            class="secondary-menu"
            in:fly={{ y: -10, duration: 200 }}
            bind:this={secondaryMenuRef}
          >
            <div class="menu-item-with-submenu">
              <button
                onclick={() => (showStatusMenu = !showStatusMenu)}
                class="menu-item"
              >
                <i class="fa-solid fa-tags"></i>
                Status
                <i class="fa-solid fa-chevron-right chevron"></i>
              </button>

              {#if showStatusMenu}
                <div
                  class="status-submenu secondary-menu"
                  in:fly={{ x: 10, duration: 200 }}
                  bind:this={statusMenuRef}
                >
                  <StatusSelector
                    categories={novel.categories}
                    {toggleStatus}
                    clearStatuses={withMenuClose(() =>
                      appState.setGameCategories(novel.id, []),
                    )}
                  />
                </div>
              {/if}
            </div>

            <div class="menu-divider"></div>

            <button onclick={withMenuClose(onEditExe)} class="menu-item">
              <i class="fa-regular fa-pen-to-square"></i>
              Edit Executable Path
            </button>

            <button onclick={withMenuClose(onProcessDialog)} class="menu-item">
              <i class="fa-solid fa-folder-tree"></i>
              Change Process Path
            </button>

            <button
              onclick={withMenuClose(() =>
                revealItemInDir(novel.exe_file_path),
              )}
              class="menu-item"
            >
              <i class="fa-solid fa-folder-open"></i>
              Open Game Directory
            </button>

            <button
              onclick={withMenuClose(() =>
                openUrl(`https://vndb.org/${novel.id}`),
              )}
              class="menu-item"
            >
              <i class="fa-solid fa-arrow-up-right-from-square"></i>
              Open in VNDB
            </button>

            <button
              onclick={withMenuClose(() =>
                invoke('set_characters', { gameId: novel.id }),
              )}
              class="menu-item"
            >
              <i class="fa-solid fa-user-plus"></i>
              Download Characters
            </button>

            <div class="menu-divider"></div>

            <button
              onclick={withMenuClose(onDeleteDialog)}
              class="menu-item danger"
            >
              <i class="fa-regular fa-trash-can"></i>
              Delete Game
            </button>
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
  }

  .blur:hover {
    filter: blur(0);
  }

  .header {
    display: flex;
    flex-direction: column;
    padding: 2rem;
    gap: 1rem;
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
    margin: 0;
    font-size: 2.5rem;
    font-weight: 700;
  }

  .alt-title {
    color: var(--main-text);
    margin: 0.25rem 0 0.5rem 0;
    font-size: 1.1rem;
    font-weight: 500;
    font-style: italic;
    opacity: 0.8;
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

  .action-buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1rem;
  }

  .primary-actions {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .more-actions-container {
    position: relative; /* Establish new positioning context */
  }

  .primary-button {
    border: 0;
    border-radius: var(--small-radius);
    color: var(--main-text);
    background: var(--primary);
    padding: 0.75rem 1.5rem;
    font-size: 18px;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 140px;
    justify-content: center;
  }

  .primary-button:hover {
    background: var(
      --primary-dark,
      color-mix(in srgb, var(--primary), #000 10%)
    );
  }

  .primary-button.playing {
    background: rgb(224, 90, 90);
  }

  .primary-button.playing:hover {
    background: color-mix(in srgb, rgb(224, 90, 90), #000 10%);
  }

  .action-button {
    border: 0;
    border-radius: var(--small-radius);
    color: var(--main-text);
    background: var(--accent);
    padding: 0.5rem 1rem;
    font-size: 16px;
    cursor: pointer;
    transition: all 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    white-space: nowrap;
  }

  .action-button:hover {
    background: color-mix(in srgb, var(--accent), var(--main-text) 10%);
  }

  .action-button.pinned {
    background: var(--secondary);
  }

  .action-button.pinned:hover {
    background: color-mix(in srgb, var(--secondary), #000 10%);
  }

  .menu-toggle {
    border: 0;
    border-radius: var(--small-radius);
    color: var(--main-text);
    background: var(--accent);
    padding: 0.5rem 1rem;
    font-size: 16px;
    cursor: pointer;
    transition: all 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    white-space: nowrap;
  }

  .menu-toggle:hover {
    background: color-mix(in srgb, var(--accent), var(--main-text) 10%);
  }

  .menu-toggle.active {
    background: var(--secondary);
  }

  .secondary-menu {
    position: absolute;
    top: 100%;
    left: 0;
    background: var(--main-background);
    border: 1px solid var(--accent);
    border-radius: var(--small-radius);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    padding: 0.5rem;
    min-width: 220px;
    z-index: 1;
    margin-top: 0.5rem;
  }

  .menu-item {
    width: 100%;
    border: 0;
    border-radius: var(--small-radius);
    color: var(--main-text);
    background: transparent;
    padding: 0.75rem 1rem;
    font-size: 14px;
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

  .menu-item.danger {
    color: #f7768e;
  }

  .menu-item.danger:hover {
    background: rgba(247, 118, 142, 0.1);
  }

  .menu-divider {
    height: 1px;
    background: var(--accent);
    margin: 0.5rem 0;
  }

  .action-button i,
  .menu-toggle i {
    font-size: 12px;
  }

  .primary-button i {
    font-size: 14px;
  }

  .menu-item i {
    font-size: 14px;
    width: 16px;
    text-align: center;
  }

  /* New styles for status submenu */
  .menu-item-with-submenu {
    position: relative;
  }

  .status-submenu {
    position: absolute;
    left: 100%;
    top: 0;
    margin-top: 0;
    margin-left: 0.5rem;
    min-width: 180px;
    z-index: 2;
  }

  .menu-item-with-submenu .menu-item {
    justify-content: space-between; /* To push chevron to the right */
  }

  .chevron {
    margin-left: auto; /* Push chevron to the right */
  }
</style>
