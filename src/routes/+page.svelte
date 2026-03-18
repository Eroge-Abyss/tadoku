<script lang="ts">
  import { gamesStore } from '$lib/stores/games.svelte';
  import GamesList from '$lib/components/home/GamesList.svelte';
  import RandomGameButton from '$lib/components/home/RandomGameButton.svelte';
</script>

<main class="container home">
  <GamesList gamesList={gamesStore.filtered} />
  <RandomGameButton />
</main>

<style>
  .home {
    position: relative;
    height: 100%;
    overflow: hidden;
  }
</style>
