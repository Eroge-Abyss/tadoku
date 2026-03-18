<script lang="ts">
  import { fly } from 'svelte/transition';
  import { elasticOut } from 'svelte/easing';
  import Card from '$lib/components/Card.svelte';
  import { appState } from '$lib/state.svelte';
  import type { Game } from '$lib/types';
  import FilterAndSort from '$lib/components/home/FilterAndSort.svelte';
  import NowPlaying from '$lib/components/home/NowPlaying.svelte';
  import { formatTime } from '$lib/util';
  import { getAvailable } from '$lib/util';

  let { gamesList }: { gamesList: Record<string, Game> } = $props();

  function getTitle(game: Game): string {
    if (appState.useJpForTitleTime) {
      const alt = getAvailable(game.alt_title);
      if (alt) return alt;
    }
    return game.title;
  }

  const totalPlaytime = $derived.by(() => {
    const seconds = Object.values(gamesList).reduce(
      (sum, game) => sum + game.playtime,
      0,
    );
    return {
      seconds,
      hours: Math.floor(seconds / 3600),
      minutes: Math.floor((seconds % 3600) / 60),
    };
  });

  let filteredGamesList = $derived.by(() => {
    if (appState.selectedCategories.length === 0) {
      return gamesList;
    }
    const filtered: Record<string, Game> = {};
    for (const id in gamesList) {
      const game = gamesList[id];
      game.categories.forEach((status) => {
        if (appState.selectedCategories.includes(status)) {
          filtered[id] = game;
        }
      });

      if (
        game.categories.length === 0 &&
        appState.selectedCategories.includes('Uncategorized')
      ) {
        filtered[id] = game;
      }
    }
    return filtered;
  });
</script>

<div class="container">
  <div class="header">
    <div class="title-area">
      <h1>Visual Novels</h1>
      {#if totalPlaytime.seconds > 0}
        <span class="total-playtime">
          <i class="fa-solid fa-clock"></i>
          {formatTime(totalPlaytime.hours, totalPlaytime.minutes)} total
        </span>
      {/if}
    </div>
    <div class="header-actions">
      <NowPlaying />
      <FilterAndSort />
    </div>
  </div>
  <div class="grid">
    {#each Object.entries(filteredGamesList) as [id, game] (id)}
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
          title={getTitle(game)}
          playtime={game.playtime}
        />
      </div>
    {/each}
  </div>
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
</style>
