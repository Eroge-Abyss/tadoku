<script>
  import CloseIcon from '$lib/util/CloseIcon.svelte';
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

<main>
  <section id="modal" class:open={isOpen}>
    <section class="modal__content">
      <header>
        <h5>{title}</h5>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <span onclick={closeModal}>
          <CloseIcon style="font-size: 24px;" />
        </span>
      </header>
      <section class="main__content">
        <p>{@html message}</p>
        <div class="buttons">
          <button onclick={onConfirm}>OK</button>
          <button onclick={closeModal} style="background: #f7768e"
            >Cancel</button
          >
        </div>
      </section>
    </section>
  </section>
</main>

<style>
  #modal {
    position: fixed;
    height: 100%;
    width: 100%;
    z-index: 2;
    top: 0;
    left: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--text-main);
    background: rgba(0, 0, 0, 0.6);
    opacity: 0;
    pointer-events: none;
    transition: all 0.2s ease-in-out;
    border-radius: 5px;
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
      width: 50%;
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
      & .main__content {
        color: var(--main-text);
        padding: 1rem;
        & .buttons {
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 1rem;
          padding: 1rem;
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
  }
</style>
