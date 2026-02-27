<script>
  import { fly } from 'svelte/transition';
  import { formatRelativeDate } from '$lib/util';
  import ProgressOverview from './ProgressOverview.svelte';
  import CharactersTab from './CharactersTab.svelte';
  import NotesTab from './NotesTab.svelte';

  let {
    novel,
    selectedTab = $bindable(),
    tabs,
    hoursPlayed,
    minutesPlayed,
    todayHoursPlayed,
    todayMinutesPlayed,
    firstPlayedDate,
    lastPlayedDate,
    jitenCharCount = null,
    jitenLoading = false,
    charsRead = 0,
    notes = $bindable(),
    editingNotes = $bindable(),
    onSaveNotes,
    onCancelEdit,
    onStartEdit,
  } = $props();
</script>

{#snippet spinner()}
  <span class="spinner">
    <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
      <circle cx="12" cy="12" r="10" />
    </svg>
  </span>
{/snippet}

<div class="tabs" in:fly={{ y: 50, duration: 500, delay: 600 }}>
  <div class="tab">
    {#each tabs as tab}
      {#if tab.visible}
        <button
          class={selectedTab == tab.id ? 'active' : ''}
          class:disabled={tab.disabled}
          disabled={tab.disabled}
          onclick={() => (selectedTab = tab.id)}
        >
          <span class="tab-label">
            {tab.label}
            {#if tab.loading}
              <span class="spinner-container">
                {@render spinner()}
              </span>
            {/if}
          </span>
        </button>
      {/if}
    {/each}
  </div>
  <div class="tab-content">
    {#if selectedTab == 'progress'}
      <ProgressOverview
        {hoursPlayed}
        {minutesPlayed}
        {todayHoursPlayed}
        {todayMinutesPlayed}
        {firstPlayedDate}
        {lastPlayedDate}
        {formatRelativeDate}
        {jitenCharCount}
        {jitenLoading}
        {charsRead}
      />
    {:else if selectedTab == 'characters'}
      {#if novel.characters && novel.characters.length > 0}
        <CharactersTab bind:characters={novel.characters} />
      {:else}
        <div class="empty-state">
          <i class="fa-solid fa-users"></i>
          <p>No Characters available</p>
        </div>
      {/if}
    {:else if selectedTab == 'notes'}
      <NotesTab
        bind:notes
        bind:editingNotes
        {onSaveNotes}
        {onCancelEdit}
        {onStartEdit}
      />
    {/if}
  </div>
</div>

<style>
  .tabs {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .tab {
    display: flex;
    gap: 1.5rem;
    justify-content: flex-start;
    align-items: center;
  }

  .tab button {
    background: var(--main-background);
    color: var(--main-text);
    border: 0;
    padding: 1rem;
    font-size: 1.5rem;
    text-align: center;
    position: relative;
    cursor: pointer;
  }

  .tab button:hover:not(:disabled)::after,
  .tab button.active::after {
    transform: scaleX(1);
    opacity: 1;
  }

  .tab button.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .tab button .tab-label {
    display: inline-flex;
    align-items: center;
  }

  .tab button .spinner-container {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-left: 0.5rem;
  }

  .tab button .spinner {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
  }

  .tab button .spinner svg {
    width: 100%;
    height: 100%;
    animation: spin 1.8s linear infinite;
  }

  .tab button .spinner circle {
    fill: none;
    stroke: var(--secondary);
    stroke-width: 3;
    stroke-linecap: round;
    stroke-dasharray: 50, 200;
    stroke-dashoffset: 0;
    animation: dash 2.5s ease-in-out infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes dash {
    0% {
      stroke-dasharray: 1, 200;
      stroke-dashoffset: 0;
    }
    50% {
      stroke-dasharray: 100, 200;
      stroke-dashoffset: -35;
    }
    100% {
      stroke-dasharray: 100, 200;
      stroke-dashoffset: -124;
    }
  }

  .tab button:after {
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

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    color: var(--secondary-text);
    text-align: center;
  }

  .empty-state i {
    font-size: 64px;
    margin-bottom: 1.5rem;
    opacity: 0.3;
  }

  .empty-state p {
    margin: 0 0 0.5rem 0;
    font-size: 18px;
    font-weight: 500;
    color: var(--main-text);
    opacity: 0.6;
  }
</style>
