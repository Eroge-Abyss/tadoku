<script>
  import { fly } from 'svelte/transition';
  import { elasticOut } from 'svelte/easing';
  import SortOrderSelect from '$lib/components/SortOrderSelect.svelte';
  import Card from '$lib/components/Card.svelte';
  import { appState } from '$lib/state.svelte';

  let { gamesList } = $props();
</script>

<div class="container">
  <div class="header">
    <h1>Visual Novels</h1>
    <SortOrderSelect />
  </div>
  <div class="grid">
    {#each Object.entries(gamesList) as [id, game]}
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
