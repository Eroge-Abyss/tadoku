<script>
  import CloseIcon from '$lib/util/CloseIcon.svelte';
  import Dialog from '$lib/util/Dialog.svelte';
  let {
    isOpen = $bindable(),
    onConfirm,
    title = 'Confirm',
    message = 'Are you sure you want to proceed?',
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
    <p>{@html message}</p>
    <div class="buttons">
      <button onclick={handleConfirm}>OK</button>
      <button onclick={closeModal} style="background: #f7768e">Cancel</button>
    </div>
  </div>
</Dialog>

<style>
  /* Styles specific to confirm dialog content */
  .confirm-dialog-content {
    color: var(--main-text);
  }

  .confirm-dialog-content p {
    margin-bottom: 1rem; /* Add some space below the message */
  }

  .confirm-dialog-content .buttons {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
  }

  .confirm-dialog-content .buttons button {
    border: 0;
    background-color: #313131;
    border-radius: var(--small-radius);
    color: #fff;
    width: 100%;
    padding: 0.5rem;
    font-size: 18px;
    margin-top: 0; /* Remove margin-top as it's on the container now */
    cursor: pointer;
  }

  .confirm-dialog-content .buttons button:hover {
    background-color: #404040; /* Add a simple hover effect */
  }
</style>
