<script lang="ts">
  import { fly } from 'svelte/transition';
  import StatusSelector from '$lib/components/StatusSelector.svelte';
  import { appState } from '$lib/state.svelte';
  import type { SortOrder } from '$lib/types';

  let menuToggleRef: HTMLButtonElement;
  // svelte-ignore non_reactive_update
  let filterSortMenuRef: HTMLDivElement;
  let activeMenu = $state(false);
  let showStatusMenu = $state(false);
  let showSortMenu = $state(false);
  let currentSortOption = $state(appState.sortOrder || 'title');

  function toggleStatus(status: string) {
    if (appState.selectedCategories.includes(status)) {
      appState.selectedCategories = appState.selectedCategories.filter(
        (s) => s !== status,
      );
    } else {
      appState.selectedCategories = [...appState.selectedCategories, status];
    }
  }

  function clearAllStatuses() {
    appState.selectedCategories = [];
  }

  function handleClickOutside(event: any) {
    // Close primary menu if click is outside it
    if (
      activeMenu &&
      filterSortMenuRef &&
      !filterSortMenuRef.contains(event.target) &&
      menuToggleRef &&
      !menuToggleRef.contains(event.target)
    ) {
      activeMenu = false;
      showStatusMenu = false; // Also close status menu if primary closes
      showSortMenu = false; // Also close sort menu if primary closes
    }

    // Close status submenu if click is outside it AND the main secondary menu,
    // but only if the main menu is still active (implies click was *inside* main menu but outside status submenu)
    if (
      showStatusMenu &&
      filterSortMenuRef &&
      !filterSortMenuRef.contains(event.target)
    ) {
      const statusToggleButton = filterSortMenuRef.querySelector(
        '.status-menu-item-toggle',
      );
      if (statusToggleButton && !statusToggleButton.contains(event.target)) {
        showStatusMenu = false;
      }
    }

    // Close sort submenu if click is outside it AND the main secondary menu
    if (
      showSortMenu &&
      filterSortMenuRef &&
      !filterSortMenuRef.contains(event.target)
    ) {
      const sortToggleButton = filterSortMenuRef.querySelector(
        '.sort-menu-item-toggle',
      );
      if (sortToggleButton && !sortToggleButton.contains(event.target)) {
        showSortMenu = false;
      }
    }
  }

  async function setSortOrder(sortOption: SortOrder) {
    currentSortOption = sortOption;
    await appState.setSortOrder(sortOption);
  }

  async function setStatuses(statuses: string[]) {}

  function withMenuClose<T extends (...args: any[]) => any>(fn: T): T {
    return ((...args: Parameters<T>): ReturnType<T> => {
      showSortMenu = false; // Close only the sort menu on selection
      return fn(...args);
    }) as T;
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="filter-sort-container">
  <button
    onclick={() => (activeMenu = !activeMenu)}
    class="menu-toggle"
    class:active={activeMenu}
    bind:this={menuToggleRef}
  >
    <i class="fa-solid fa-filter"></i>
    Filter & Sort
  </button>

  {#if activeMenu}
    <div
      class="secondary-menu"
      in:fly={{ y: -10, duration: 200 }}
      bind:this={filterSortMenuRef}
    >
      <h3>Sort By</h3>
      <div class="menu-item-with-submenu">
        <button
          onclick={() => (showSortMenu = !showSortMenu)}
          class="menu-item sort-menu-item-toggle"
        >
          <i class="fa-solid fa-arrow-down-short-wide"></i>
          {currentSortOption.charAt(0).toUpperCase() +
            currentSortOption.slice(1).replace('_', ' ')}
          <i class="fa-solid fa-chevron-right chevron"></i>
        </button>

        {#if showSortMenu}
          <div
            class="sort-submenu secondary-menu"
            in:fly={{ x: 10, duration: 200 }}
          >
            <button
              onclick={withMenuClose(() => setSortOrder('title'))}
              class="menu-item"
              class:active={currentSortOption === 'title'}
            >
              Title
            </button>
            <button
              onclick={withMenuClose(() => setSortOrder('last_played'))}
              class="menu-item"
              class:active={currentSortOption === 'last_played'}
            >
              Last Played
            </button>
            <button
              onclick={withMenuClose(() => setSortOrder('playtime'))}
              class="menu-item"
              class:active={currentSortOption === 'playtime'}
            >
              Playtime
            </button>
          </div>
        {/if}
      </div>

      <div class="menu-divider"></div>

      <h3>Filter By</h3>
      <div class="menu-item-with-submenu">
        <button
          onclick={() => (showStatusMenu = !showStatusMenu)}
          class="menu-item status-menu-item-toggle"
        >
          <i class="fa-solid fa-tags"></i>
          Status
          <i class="fa-solid fa-chevron-right chevron"></i>
        </button>

        {#if showStatusMenu}
          <div
            class="status-submenu secondary-menu"
            in:fly={{ x: 10, duration: 200 }}
          >
            <StatusSelector
              categories={appState.selectedCategories}
              {toggleStatus}
              clearStatuses={clearAllStatuses}
            />
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .filter-sort-container {
    position: relative;
    display: inline-block;
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
    right: 0;
    background: var(--main-background);
    border: 1px solid var(--accent);
    border-radius: var(--small-radius);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    padding: 0.5rem;
    min-width: 220px;
    z-index: 1;
    margin-top: 0.5rem;
  }

  .secondary-menu h3 {
    color: var(--foreground);
    font-size: 1rem;
    margin: 0.5rem 1rem;
    font-weight: 600;
    opacity: 0.8;
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

  .menu-item.active {
    background: var(--secondary);
    font-weight: 600;
  }

  .menu-divider {
    height: 1px;
    background: var(--accent);
    margin: 0.5rem 0;
  }

  .menu-toggle i {
    font-size: 12px;
  }

  .menu-item i {
    font-size: 14px;
    width: 16px;
    text-align: center;
  }

  /* Submenu positioning and styling */
  .menu-item-with-submenu {
    position: relative;
  }

  .status-submenu,
  .sort-submenu,
  .status-submenu {
    position: absolute;
    right: 100%; /* Open to the left of the parent menu item */
    top: 0;
    margin-top: 0;
    margin-right: 0.5rem; /* Space between parent and submenu */
    min-width: 180px;
    z-index: 2;
  }

  .menu-item-with-submenu .menu-item {
    justify-content: space-between;
  }

  .chevron {
    margin-left: auto;
  }
</style>
