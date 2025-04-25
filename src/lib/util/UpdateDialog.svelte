<script lang="ts">
  import CloseIcon from '$lib/util/CloseIcon.svelte';
  import { check, Update } from '@tauri-apps/plugin-updater';
  import { onMount } from 'svelte';

  let loading = $state(false);
  let update = $state<Update | null>(null);
  let isOpen = $state(false);

  onMount(() => {
    check()
      .then(async (maybeUpdate) => {
        if (!maybeUpdate) {
          return;
        }

        update = maybeUpdate;

        // AI suggesstion to fix model opening transition
        // it's because without the timeout, the browser does not have a chance
        // to render this component in `isOpen = false` state
        // so using the timeout makes `isOpen = true` in another execution cycle
        setTimeout(() => {
          isOpen = true;
        }, 50);
      })
      .catch(console.error);
  });

  // Close the modal
  function closeModal() {
    isOpen = false;
  }

  // Handle the "Install Now" button click
  async function installNow() {
    loading = true;
    try {
      if (!update) {
        closeModal();
        return;
      }

      await update.download();
      await update.install();
    } catch (error) {
      console.error('Failed to install update:', error);
    } finally {
      loading = false;
      closeModal();
    }
  }

  function installLater() {
    closeModal();
  }
</script>

{#if update && update.version}
  <section class="modal" class:open={isOpen}>
    <section class="modal__content">
      <header>
        <h3>Update Available</h3>
        <span onclick={closeModal}>
          <CloseIcon style="font-size: 24px;" />
        </span>
      </header>
      <section class="update-form">
        <div class="update-info">
          <h4>Version {update.version} is available</h4>

          {#if update.body}
            <div class="update-notes">
              <h5>What's new:</h5>
              <p>{update.body}</p>
            </div>
          {/if}

          <div class="info-container">
            <span class="icon-info">
              <i class="fa-solid fa-info-circle"></i>
            </span>
            <p class="note">
              You can install the update now, or get prompted to install the
              next time you start the application.
            </p>
          </div>
        </div>

        <div class="btn-row">
          <button
            disabled={loading}
            class="update-now-button"
            onclick={installNow}
          >
            {#if loading}
              Installing...
            {:else}
              Install Now
            {/if}
          </button>
          <button onclick={installLater}>Install Later</button>
        </div>
      </section>
    </section>
  </section>
{/if}

<style>
  .modal {
    position: fixed;
    height: 100%;
    width: 100%;
    z-index: 3;
    top: 0;
    left: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    color: #fff;
    background: rgba(0, 0, 0, 0.6);
    opacity: 0;
    pointer-events: none;
    transition: all 0.2s ease-in-out;
    /* Start scaled down */
    &.open {
      opacity: 1;
      pointer-events: all;

      & .modal__content {
        transform: translate(0, 0) scale(1); /* Scale up */
      }
    }

    & .modal__content {
      background-color: var(--main-background);
      padding: 1rem;
      width: 500px;
      display: flex;
      flex-direction: column;
      transform: translate(0, 100%) scale(0.8);
      transition: all 0.2s ease-in-out;
      & header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        color: var(--main-text);
        span {
          color: #444;
          cursor: pointer;
          transition: color 0.2s ease-in-out;
          &:hover {
            color: var(--main-text);
          }
        }
      }

      & .update-form {
        margin: 1rem;

        & button {
          border: 0;
          background-color: #313131;
          color: #fff;
          width: 100%;
          padding: 0.5rem;
          font-size: 18px;
          margin-top: 1rem;
          cursor: pointer;
        }
      }
    }
  }

  .update-now-button {
    background: #9ece6a !important;
    &[disabled] {
      opacity: 0.5;
      cursor: not-allowed;
    }
  }

  .update-info {
    padding: 10px;
    color: var(--main-text);
  }

  h4 {
    margin-top: 0;
    margin-bottom: 1rem;
  }

  .update-notes {
    background: #313131;
    padding: 10px;
    border-radius: var(--small-radius);
    margin-bottom: 1rem;
  }

  .update-notes h5 {
    margin-top: 0;
    margin-bottom: 0.5rem;
    color: var(--main-text);
  }

  .update-notes p {
    margin: 0;
    white-space: pre-line;
    font-size: 14px;
  }

  .note {
    font-size: 12px;
    color: var(--secondary-text);
    margin: 0;
  }

  .info-container {
    display: flex;
    padding: 10px 0;
    align-items: flex-start;
  }

  .icon-info {
    font-size: 14px;
    margin-right: 5px;
    color: var(--secondary-text);
  }

  .btn-row {
    display: flex;
    flex-direction: row-reverse;
    gap: 1rem;
  }
</style>
