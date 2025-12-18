<script>
  import Dialog from '$lib/components/Dialog.svelte';
  let {
    isOpen = $bindable(),
    onConfirm,
    title = 'Confirm Action',
    message = 'Are you sure you want to proceed?',
    confirmText = 'Confirm',
    cancelText = 'Cancel',
    isDanger = false,
  } = $props();

  // Close the modal
  function closeModal() {
    isOpen = false;
  }

  // Handle the "OK" button click
  function handleConfirm() {
    if (onConfirm) {
      onConfirm(); // Run the provided function
    }
    closeModal(); // Close the modal
  }
</script>

<Dialog show={isOpen} close={closeModal}>
  {#snippet header()}
    {title}
  {/snippet}

  <div class="confirm-dialog-content">
    <div class="message">
      {@html message}
    </div>
    <div class="buttons">
      <button class="cancel-btn" onclick={closeModal}>
        {cancelText}
      </button>
      <button
        class="confirm-btn"
        class:danger={isDanger}
        onclick={handleConfirm}
      >
        {confirmText}
      </button>
    </div>
  </div>
</Dialog>

<style>
  .confirm-dialog-content {
    color: var(--main-text);
  }

  .message {
    margin-bottom: 1.5rem;
    font-size: 15px;
    line-height: 1.6;
    text-align: center;
    color: var(--secondary-text);
  }

  /* Themed highlights for text in messages */
  :global(.message b),
  :global(.message strong) {
    color: var(--secondary);
    font-weight: 700;
  }

  :global(.message .danger-highlight) {
    color: #f7768e;
    font-weight: 700;
  }

  .buttons {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  button {
    border: 0;
    border-radius: var(--small-radius);
    color: var(--main-text);
    width: 100%;
    padding: 0.5rem;
    font-size: 18px;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .cancel-btn {
    background-color: var(--accent);
  }

  .cancel-btn:hover {
    background-color: color-mix(in srgb, var(--accent), white 10%);
  }

  .confirm-btn {
    background-color: var(--primary);
  }

  .confirm-btn:hover {
    background-color: color-mix(in srgb, var(--primary), #000 10%);
  }

  .confirm-btn.danger {
    background-color: #f7768e;
  }

  .confirm-btn.danger:hover {
    background-color: color-mix(in srgb, #f7768e, #000 10%);
  }
</style>
