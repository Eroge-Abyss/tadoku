<script lang="ts">
  import { fly } from 'svelte/transition';
  import { elasticOut } from 'svelte/easing';
  import Card from '$lib/components/Card.svelte';
  import type { Game } from '$lib/types';
  import FilterAndSort from '$lib/components/home/FilterAndSort.svelte';
  import NowPlaying from '$lib/components/home/NowPlaying.svelte';
  import { formatTime, getPreferredTitle } from '$lib/util';
  import { gamesStore } from '$lib/stores/games.svelte';

  let { gamesList }: { gamesList: Record<string, Game> } = $props();
</script>

<div class="container">
  <div class="header">
    <div class="title-area">
      <h1>Visual Novels</h1>
      {#if gamesStore.totalPlaytime.seconds > 0}
        <span class="total-playtime">
          <i class="fa-solid fa-clock"></i>
          {formatTime(
            gamesStore.totalPlaytime.hours,
            gamesStore.totalPlaytime.minutes,
          )} total
        </span>
      {/if}
    </div>
    <div class="header-actions">
      <NowPlaying />
      <FilterAndSort />
    </div>
  </div>

  {#if Object.keys(gamesList).length === 0}
    <div class="empty-state">
      <i class="fa-solid fa-gamepad empty-icon"></i>
      <h2>Your library is empty</h2>
      <p>
        Click the <span class="plus-badge"
          ><i class="fa-solid fa-plus"></i></span
        > button to add your first game.
      </p>
    </div>
  {:else}
    <div class="grid">
      {#each Object.entries(gamesList) as [id, game] (id)}
        <div
          in:fly={{
            y: 50,
            duration: 500,
            delay: 100,
            easing: elasticOut,
          }}
        >
          <Card
            {id}
            image={game.image_url}
            isNsfw={game.is_nsfw}
            title={getPreferredTitle(game)}
            playtime={game.playtime}
          />
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .container {
    padding-left: 3rem;
    display: flex;
    flex-direction: column;
    height: 100%;
    box-sizing: border-box;
    overflow: hidden;
  }

  h1 {
    color: var(--foreground);
    font-size: 2.5rem;
    font-weight: 700;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-right: 2rem;
    margin-bottom: 1rem;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .title-area {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .total-playtime {
    font-size: 1rem;
    color: var(--secondary-text);
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .total-playtime i {
    font-size: 0.75rem;
    opacity: 0.8;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1.5rem;
    width: 100%;
    flex: 1;
    overflow-y: auto;
    padding-bottom: 2rem;
    padding-right: 2rem;
    padding-top: 1rem;
  }

  /* --- Empty State Styles --- */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    text-align: center;
    padding-right: 2rem; /* Matches the grid/header padding to stay centered visually */
    color: var(--secondary-text);
  }

  .empty-icon {
    font-size: 3.5rem;
    margin-bottom: 1rem;
    opacity: 0.3;
  }

  .empty-state h2 {
    color: var(--foreground);
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
  }

  .empty-state p {
    font-size: 1rem;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    opacity: 0.8;
  }

  .plus-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border-radius: 4px;
    background-color: color-mix(
      in srgb,
      var(--secondary-text) 15%,
      transparent
    );
    color: var(--foreground);
    font-size: 0.8rem;
  }

  .plus-badge i {
    line-height: 0;
    transform: translateY(0.5px);
  }

  :global(.animate-spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
