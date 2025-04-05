<script>
  import Button from '$lib/util/Button.svelte'
  import AddGame from '$lib/AddGame.svelte'
  import { goto } from '$app/navigation'
  import { appState } from '../routes/state.svelte'
  import { convertFileSrc, invoke } from '@tauri-apps/api/core'
  import SquaresIcon from '$lib/util/SquaresIcon.svelte'
  import SettingsButton from './util/SettingsButton.svelte'
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
        <Button
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
</nav>

<style>
  nav {
    height: 100vh;
    width: 85px;
    padding-bottom: 1rem;
    background-color: var(--accent);
    color: var(--main-text);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
    position: sticky;
    top: -1px;
    z-index: 3;
    border-right: 1px solid rgba(255, 255, 255, 0.05);
    box-shadow: 1px 0 10px rgba(0, 0, 0, 0.15);
    transition: background-color 0.3s ease;
    
    #sidebar__header {
      width: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;
      position: relative;
      
      h1 {
        color: var(--main-mauve);
        font-size: 50px;
        padding: 1rem;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        transition: color 0.3s ease;
        opacity: 0.8;

        &:hover {
          opacity: 1;
        }
      }

      #sidebar__header__buttons {
        text-align: center;
        margin-top: 2rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;
        

        :global(button) {
          background: rgba(255, 255, 255, 0.08);
          border: 1px solid rgba(255, 255, 255, 0.1);
          transition: all 0.3s ease;
          box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);

          &:hover {
            transform: translateY(-2px);
            background: rgba(255, 255, 255, 0.12);
            box-shadow: 0 6px 10px rgba(0, 0, 0, 0.12);
          }

          &:active {
            transform: translateY(0);
          }
        }
      }
    }
    
    #sidebar__footer {
      margin-top: auto;
      width: 100%;
      display: flex;
      justify-content: center;
      
      
      :global(button) {
        background: rgba(255, 255, 255, 0.08);
        border: 1px solid rgba(255, 255, 255, 0.1);
        transition: all 0.3s ease;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        width: 100%;

        &:hover {
          transform: translateY(-2px);
          background: rgba(255, 255, 255, 0.12);
          box-shadow: 0 6px 10px rgba(0, 0, 0, 0.12);
        }

        &:active {
          transform: translateY(0);
        }
      }
    }
  }
</style>