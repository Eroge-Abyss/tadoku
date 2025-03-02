<script>
  import Button from '$lib/util/Button.svelte';
  import AddGame from '$lib/AddGame.svelte';
  // import Icon from "@iconify/svelte";
  import { goto } from '$app/navigation';
  import { appState } from '../routes/state.svelte';
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import SquaresIcon from '$lib/util/SquaresIcon.svelte';

  let pinnedGames = $derived.by(() =>
    Object.entries(appState.gamesList)
      .filter(([k, v]) => v.is_pinned)
      .map(([k, v]) => ({
        id: k,
        char: v.title[0],
        image: v.icon_url ? convertFileSrc(v.icon_url) : null,
      })),
  );
</script>

<nav>
  <section id="sidebar__header">
    <h1>多</h1>
    <div id="sidebar__header__buttons">
      <Button onclick={() => goto('/')}>
        <SquaresIcon style="font-size: 24px;" />
      </Button>

      {#each pinnedGames as { id, image, char } (id)}
        {console.log(pinnedGames)}
        <Button
          onclick={() => invoke('open_game', { gameId: id })}
          image={image ? image : undefined}
          text={image ? undefined : char}
        />
      {/each}

      <AddGame />
    </div>
  </section>

  <!-- TODO: Hidden until implemented for completeness -->
  <!-- <Icon
        icon="material-symbols:settings-outline-rounded"
        style="font-size: 24px; color: #fff; cursor:pointer;"
    /> -->
</nav>

<style>
  nav {
    height: 100vh;
    width: 85px;
    padding-bottom: 1rem;
    background: #1b1b1b;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    position: sticky;
    top: -1px;
    /*كل ما تعطل اديها */
    z-index: 3;
    #sidebar__header {
      h1 {
        color: #5d5d5d;
        font-size: 50px;
        padding: 1rem;
      }

      #sidebar__header__buttons {
        text-align: center;
        margin-top: 2.5rem;
        /*كل ما تعطل اديها */
        display: flex;
        flex-direction: column;
        gap: 1rem;
      }
    }
  }
</style>
