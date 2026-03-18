<script lang="ts">
  import Dialog from '$lib/components/Dialog.svelte';
  import SidebarButton from '$lib/components/SidebarButton.svelte';
  import AddVndbGame from './AddVndbGame.svelte';
  import AddManualGame from './AddManualGame.svelte';

  let showModal = $state(false);
  let mode: 'vndb' | 'manual' = $state('vndb');

  const openModal = () => (showModal = true);

  const closeModal = () => {
    showModal = false;
    mode = 'vndb';
  };
</script>

<section>
  <SidebarButton onclick={openModal} tooltip="Add Game">
    <span class="add-icon">+</span>
  </SidebarButton>

  <Dialog show={showModal} close={closeModal}>
    {#snippet header()}
      Add a game
    {/snippet}

    <section class="game-form">
      <!-- Mode toggle -->
      <div class="mode-toggle">
        <button
          class="mode-btn"
          class:active={mode === 'vndb'}
          onclick={() => (mode = 'vndb')}
        >
          VNDB Search
        </button>
        <button
          class="mode-btn"
          class:active={mode === 'manual'}
          onclick={() => (mode = 'manual')}
        >
          Manual Entry
        </button>
      </div>

      {#if mode === 'vndb'}
        <AddVndbGame {closeModal} />
      {:else}
        <AddManualGame {closeModal} />
      {/if}
    </section>
  </Dialog>
</section>

<style>
  .add-icon {
    font-size: 2.5rem;
    color: var(--primary);
  }

  .game-form {
    margin: 1rem;
  }

  .mode-toggle {
    display: flex;
    gap: 0;
    margin-bottom: 1rem;
    border-radius: var(--small-radius);
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .mode-btn {
    flex: 1;
    padding: 0.4rem 0;
    border: 0;
    background: var(--accent);
    color: var(--secondary-text);
    font-size: 14px;
    cursor: pointer;
    transition:
      background 0.2s ease,
      color 0.2s ease;
  }

  .mode-btn.active {
    background: var(--primary);
    color: #fff;
  }

  .mode-btn:not(.active):hover {
    background: color-mix(in srgb, var(--accent), white 8%);
    color: var(--main-text);
  }
</style>
