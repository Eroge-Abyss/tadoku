<script>
  import CloseIcon from '$lib/components/CloseIcon.svelte';
  let { show, close, header, children } = $props(); // Prop to control visibility

  // @ts-ignore
  function handleModalClick(e) {
    // Check if the click occurred directly on the modal backdrop
    if (e.target?.classList.contains('modal')) {
      close();
    }
  }

  function handleCloseClick() {
    close();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<section class="modal" class:open={show} onmousedown={handleModalClick}>
  <section class="modal__content">
    <header>
      <h3 class="title">
        {@render header()}
      </h3>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <span onclick={handleCloseClick}>
        <CloseIcon style="font-size: 24px;" />
      </span>
    </header>
    <section class="dialog-content">
      {@render children()}
    </section>
  </section>
</section>

<style>
  .modal {
    position: fixed;
    height: 100%;
    width: 100%;
    z-index: 2; /* Adjust if necessary */
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

    /* Start scaled down or off-screen */
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
      width: 500px; /* Adjust width as needed */
      max-width: 90%; /* Add max-width for smaller screens */
      border-radius: var(--big-radius);
      display: flex;
      flex-direction: column;
      transform: translate(0, 100%) scale(0.8);
      transition: all 0.2s ease-in-out;
      box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1); /* Optional shadow */

      & header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        color: var(--main-text);
        padding: 0 10px; /* Add some padding to header */

        & .title {
          margin: 0; /* Remove default margin from h3 */
          font-size: 1.5rem; /* Adjust title size */
        }

        span {
          color: #444; /* Color for close icon */
          cursor: pointer;
          transition: color 0.2s ease-in-out;
          &:hover {
            color: var(--main-text);
          }
        }
      }

      & .dialog-content {
        margin: 1rem;
        /* Add padding or styling for the content area if needed */
      }
    }
  }
</style>
