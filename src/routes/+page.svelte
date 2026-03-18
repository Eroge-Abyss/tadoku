<script lang="ts">
  import { onMount } from 'svelte';
  import { appState } from '$lib/state.svelte';
  import GamesList from '$lib/components/home/GamesList.svelte';
  import RandomGameButton from '$lib/components/home/RandomGameButton.svelte';
  onMount(async () => {
    await appState.loadGames();
  });
</script>

<main class="container home">
  <GamesList gamesList={appState.gamesList} />
  <RandomGameButton />
</main>

<style>
  .home {
    position: relative;
    height: 100%;
    overflow: hidden;
  }
</style>
