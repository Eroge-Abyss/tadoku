<script>
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { fly } from 'svelte/transition';
  import { openUrl } from '@tauri-apps/plugin-opener';

  let { characters = $bindable() } = $props();
</script>

<div class="characters" in:fly={{ y: 20, duration: 300 }}>
  {#each characters as character}
    <div
      class="character-card"
      onclick={() => openUrl(`https://vndb.org/${character.id}`)}
    >
      {#if character.image_url}
        <img src={convertFileSrc(character.image_url)} alt={character.id} />
      {:else}
        <p>No Image</p>
      {/if}
      <div class="character-content">
        <p class="main">{character.og_name}</p>
        <p class="sub">{character.en_name}</p>
      </div>
      <i class="fa-solid fa-arrow-up-right-from-square"></i>
    </div>
  {/each}
</div>

<style>
  .characters {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
    gap: 1rem;
    grid-auto-rows: 1fr;
    padding: 1rem;
  }

  .character-card {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    cursor: pointer;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
    transition:
      box-shadow 0.3s ease,
      transform 0.3s ease;
    background: var(--accent);
    border-radius: 8px;
  }

  .character-content {
    padding: 1rem;
    text-align: justify;
  }

  .character-content p.main {
    color: var(--main-text);
  }

  .character-content p.sub {
    color: var(--secondary-text);
    font-size: 14px;
  }

  .character-card img {
    height: 100px;
    width: 100px;
    object-fit: cover;
    object-position: top;
    border-radius: 8px 0 0 8px;
  }

  .character-card > p {
    width: 100px;
    text-align: center;
  }

  .character-card i {
    margin-left: auto;
    padding: 1.5rem;
    color: var(--secondary-text);
  }

  .character-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 12px 20px rgba(0, 0, 0, 0.4);
  }

  .character-card:hover i {
    color: var(--main-text);
  }
</style>
