<script lang="ts">
  import { fly } from 'svelte/transition';
  import { elasticOut } from 'svelte/easing';
  import Card from '$lib/components/Card.svelte';
  import { appState } from '$lib/state.svelte';
  import type { Game } from '$lib/types';
  import FilterAndSort from '$lib/components/home/FilterAndSort.svelte';

  let { gamesList }: { gamesList: Record<string, Game> } = $props();

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
    <h1>Visual Novels</h1>
    <FilterAndSort />
  </div>
  <div class="grid">
    {#each Object.entries(filteredGamesList) as [id, game]}
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
          title={appState.useJpForTitleTime && game.alt_title
            ? game.alt_title
            : game.title}
          playtime={game.playtime}
        />
      </div>
    {/each}
  </div>
</div>

<style>
  .container {
    padding: 2rem;
    display: flex;
    flex-direction: column;
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
    margin-bottom: 2rem;
  }
</style>
