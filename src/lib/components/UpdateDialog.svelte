<script lang="ts">
  import { check, Update } from '@tauri-apps/plugin-updater';
  import { onMount } from 'svelte';
  import Dialog from '$lib/components/Dialog.svelte';
  import InfoNote from './InfoNote.svelte';

  const GITHUB_RELEASE_URL =
    'https://github.com/Eroge-Abyss/tadoku/releases/latest';

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

        // AI suggestion to fix model opening transition
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
  <Dialog show={isOpen} close={closeModal}>
    {#snippet header()}
      Update Available
    {/snippet}

    <section class="update-form">
      <div class="update-info">
        <h4>Version {update.version} is available</h4>

        {#if update.body}
          <div class="update-notes">
            <h5>What's new:</h5>
            <p>
              Check <a
                href={GITHUB_RELEASE_URL}
                target="_blank"
                rel="noopener noreferer">Release Notes</a
              > for details.
            </p>
          </div>
        {/if}

        <InfoNote>
          You can install the update now, or get prompted to install the next
          time you start the application.
        </InfoNote>
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
  </Dialog>
{/if}

<style>
  .update-form {
    margin: 1rem;
  }

  .update-form button {
    border: 0;
    background-color: #313131;
    border-radius: var(--small-radius); /* Apply border-radius */
    color: #fff;
    width: 100%;
    padding: 0.5rem;
    font-size: 18px;
    margin-top: 1rem; /* Keep margin-top */
    cursor: pointer;
    transition: background-color 0.3s ease; /* Keep transition */
  }

  .update-form button:hover {
    background-color: #404040; /* Match hover style */
  }

  .update-now-button {
    background-color: var(--primary) !important; /* Use primary color */
    &[disabled] {
      opacity: 0.5;
      cursor: not-allowed;
    }
    &:hover:not([disabled]) {
      background: var(
        --primary-dark,
        color-mix(in srgb, var(--primary), #000 10%)
      ) !important; /* Match hover style for primary button */
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

  .btn-row {
    display: flex;
    flex-direction: row-reverse;
    gap: 1rem;
  }
</style>
