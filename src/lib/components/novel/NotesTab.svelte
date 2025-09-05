<script>
  import { fly } from 'svelte/transition';
  import { formatNotes } from '$lib/util';

  let {
    notes = $bindable(),
    editingNotes = $bindable(),
    onSaveNotes,
    onCancelEdit,
    onStartEdit,
  } = $props();
</script>

<div class="notes" in:fly={{ y: 20, duration: 300 }}>
  {#if editingNotes}
    <textarea class="notes-div" bind:value={notes}></textarea>
    <div class="notes-actions">
      <button class="save-button" onclick={onSaveNotes} title="Save Notes">
        <i class="fa-solid fa-save"></i> Save
      </button>
      <button
        class="cancel-button"
        onclick={onCancelEdit}
        title="Cancel Editing"
      >
        <i class="fa-solid fa-xmark"></i> Cancel
      </button>
    </div>
  {:else}
    <div class="notes-div">
      {@html formatNotes(notes)}
    </div>
    <div class="notes-actions">
      <button
        class="edit-notes-button"
        onclick={onStartEdit}
        title="Edit Notes"
      >
        <i class="fa-regular fa-pen-to-square"></i> Edit Notes
      </button>
    </div>
  {/if}
</div>

<style>
  .notes-div {
    width: 100%;
    height: 300px;
    resize: none;
    padding: 10px;
    margin: 0;
    background-color: var(--accent);
    color: var(--main-text);
    border: 1px solid var(--secondary);
    border-radius: var(--small-radius);
    font-size: 1.1rem;
    outline: none;
    overflow-x: auto;
    overflow-y: auto;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  .notes-div:focus {
    border-color: var(--secondary);
    box-shadow: 0 0 0 2px rgba(187, 154, 247, 0.5);
  }

  textarea.notes-div {
    font-family: inherit;
    display: block;
    box-sizing: border-box;
    line-height: inherit;
    white-space: pre-wrap;
    word-wrap: break-word;
    overflow-wrap: break-word;
  }

  .save-button,
  .cancel-button,
  .edit-notes-button {
    border: 0;
    border-radius: var(--small-radius);
    color: var(--main-text);
    padding: 0.5rem 1rem;
    font-size: 16px;
    cursor: pointer;
    transition: all 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
  }

  .save-button {
    background: var(--primary);
  }

  .save-button:hover {
    background: var(
      --primary-dark,
      color-mix(in srgb, var(--primary), #000 10%)
    );
  }

  .cancel-button {
    background: var(--accent);
  }

  .cancel-button:hover {
    background: color-mix(in srgb, var(--accent), var(--main-text) 10%);
  }

  .edit-notes-button {
    background: var(--secondary);
  }

  .edit-notes-button:hover {
    background: color-mix(in srgb, var(--secondary), #000 10%);
  }

  .notes-actions {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
    justify-content: flex-start;
  }
</style>
