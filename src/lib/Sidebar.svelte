<script>
  import SidebarButton from '$lib/util/SidebarButton.svelte';
  import AddGame from '$lib/AddGame.svelte';
  import { goto } from '$app/navigation';
  import { appState } from '../routes/state.svelte';
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import SquaresIcon from '$lib/util/SquaresIcon.svelte';
  import SettingsButton from './util/SettingsButton.svelte';

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
      <SidebarButton onclick={() => goto('/')}>
        <SquaresIcon style="font-size: 24px;" />
      </SidebarButton>

      {#each pinnedGames as { id, image, char } (id)}
        <SidebarButton
          onclick={() => invoke('open_game', { gameId: id })}
          image={image ? image : undefined}
          text={image ? undefined : char}
        />
      {/each}

      <AddGame />
    </div>
  </section>

  <section id="sidebar__footer">
    <SettingsButton />
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
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    position: sticky;
    background-color: var(--accent);
    color: var(--main-text);
    border-right: 1px solid rgba(255, 255, 255, 0.05);
    box-shadow: 1px 0 10px rgba(0, 0, 0, 0.15);
    transition: background-color 0.3s ease;
    top: -1px;
    /*كل ما تعطل اديها */
    z-index: 3;
    #sidebar__header {
      h1 {
        color: var(--secondary); /* apply theme color to => 多 */
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
  #sidebar__footer {
    margin-top: auto;
    width: 100%;
    display: flex;
    justify-content: center;
  }
</style>
