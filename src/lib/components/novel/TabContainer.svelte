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
    notes = $bindable(),
    editingNotes = $bindable(),
    onSaveNotes,
    onCancelEdit,
    onStartEdit,
  } = $props();
</script>

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
      <ProgressOverview
        {hoursPlayed}
        {minutesPlayed}
        {todayHoursPlayed}
        {todayMinutesPlayed}
        {firstPlayedDate}
        {lastPlayedDate}
        {formatRelativeDate}
      />
    {:else if selectedTab == 'characters'}
      <CharactersTab characters={novel.characters} />
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

  .tab button:hover::after,
  .tab button.active::after {
    transform: scaleX(1);
    opacity: 1;
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
</style>
