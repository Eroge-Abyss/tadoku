<script lang="ts">
  import { gamesStore } from '$lib/stores/games.svelte';
  import { settingsStore } from '$lib/stores/settings.svelte';
  import { resolve } from '$app/paths';
  import { goto } from '$app/navigation';

  const getRandomGame = () => {
    let games = Object.entries(gamesStore.list);
    const selectedCategories = settingsStore.selectedCategories;
    if(selectedCategories.length) {
      games = games.filter(game => 
          game[1].categories.some(c => selectedCategories.includes(c)
        ));
    }
    if (games.length === 0) return;
    const randomIndex = Math.floor(Math.random() * games.length);
    const selectedGame = games[randomIndex][0];
    goto(resolve(`/novel/${selectedGame}`));
  };
</script>

{#if settingsStore.showRandomButton}
  <button
    class="fa-dice-shake random-button"
    onclick={getRandomGame}
    aria-label="random-game-button"
    title="Pick a random game"
  >
    <i class="fa-solid fa-dice"></i>
  </button>
{/if}

<style>
  .random-button {
    height: 50px;
    width: 50px;
    border-radius: 50%;
    border: none;
    background: var(--accent);
    color: var(--main-text);
    cursor: pointer;
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    transition: all 0.3s ease;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
    z-index: 50;
  }

  .random-button:hover {
    transform: translateY(-5px);
    box-shadow: 0 12px 20px rgba(0, 0, 0, 0.4);
  }

  @keyframes diceShake {
    0% {
      transform: translate(0, 0) rotate(0deg);
    }
    10% {
      transform: translate(-3px, -2px) rotate(5deg);
    }
    20% {
      transform: translate(4px, 1px) rotate(-8deg);
    }
    30% {
      transform: translate(-2px, 3px) rotate(10deg);
    }
    40% {
      transform: translate(5px, -1px) rotate(-12deg);
    }
    50% {
      transform: translate(-4px, -3px) rotate(15deg);
    }
    60% {
      transform: translate(3px, 2px) rotate(-10deg);
    }
    70% {
      transform: translate(-1px, -4px) rotate(8deg);
    }
    80% {
      transform: translate(2px, 5px) rotate(-5deg);
    }
    90% {
      transform: translate(-3px, -2px) rotate(3deg);
    }
    100% {
      transform: translate(0, 0) rotate(0deg);
    }
  }

  .fa-dice-shake:hover i {
    animation: diceShake 1s cubic-bezier(0.36, 0.07, 0.19, 0.97) both infinite;
  }
</style>
