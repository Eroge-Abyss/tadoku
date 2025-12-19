<script lang="ts">
  let {
    checked = $bindable(),
    id,
    label,
    onchange,
  }: {
    checked: boolean;
    id: string;
    label?: string;
    onchange?: (event: Event) => void;
  } = $props();
</script>

<label class="custom-checkbox-component">
  <input type="checkbox" {id} bind:checked {onchange} />
  <span class="checkmark-component"></span>
  {#if label}
    <span class="checkbox-label">{label}</span>
  {/if}
</label>

<style>
  /* Style for the custom checkbox */
  .custom-checkbox-component {
    position: relative;
    display: flex;
    align-items: center;
    cursor: pointer;
    user-select: none; /* Prevent text selection */
    gap: 0.5rem; /* Space between checkbox and label */
    color: var(--main-text);
  }

  .custom-checkbox-component input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .checkmark-component {
    position: relative;
    display: inline-block;
    height: 16px;
    width: 16px;
    background-color: var(--accent);
    border: 2px solid color-mix(in srgb, var(--accent), white 20%);
    border-radius: var(--small-radius);
    transition:
      background-color 0.3s ease,
      border-color 0.3s ease;
  }

  .custom-checkbox-component input:checked ~ .checkmark-component {
    background-color: var(--primary);
    border-color: var(--primary); /* Match border to background when checked */
  }

  .checkmark-component:after {
    content: '';
    position: absolute;
    display: none;
  }

  .custom-checkbox-component input:checked ~ .checkmark-component:after {
    display: block;
  }

  .custom-checkbox-component .checkmark-component:after {
    left: 4px;
    top: 1px;
    width: 4px;
    height: 8px;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
  }

  .checkbox-label {
    font-size: 1rem;
  }
</style>
