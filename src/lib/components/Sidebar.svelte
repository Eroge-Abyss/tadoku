<script>
  import SidebarButton from '$lib/components/SidebarButton.svelte';
  import AddGame from '$lib/components/home/AddGame.svelte';
  import { goto } from '$app/navigation';
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import SquaresIcon from '$lib/components/SquaresIcon.svelte';
  import { appState } from '$lib/state.svelte';
  import SettingsButton from '$lib/components/SettingsButton.svelte';

  let pinnedGames = $derived.by(() =>
    Object.entries(appState.gamesList)
      .filter(([_, v]) => v.is_pinned)
      .map(([k, v]) => ({
        id: k,
        char: v.title[0],
        title: v.title,
        image: v.icon_url ? convertFileSrc(v.icon_url) : null,
      })),
  );
</script>

<nav>
  <section id="sidebar__header">
    <h1>多</h1>
    <div id="sidebar__header__buttons">
      <SidebarButton onclick={() => goto('/')} tooltip="Library">
        <SquaresIcon style="font-size: 24px;" />
      </SidebarButton>

      {#each pinnedGames as { id, image, char, title } (id)}
        <SidebarButton
          onclick={() => invoke('open_game', { gameId: id })}
          image={image ? image : undefined}
          text={image ? undefined : char}
          tooltip={title}
        />
      {/each}

      <AddGame />
    </div>
  </section>

  <section id="sidebar__footer">
    <SettingsButton />
  </section>
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
