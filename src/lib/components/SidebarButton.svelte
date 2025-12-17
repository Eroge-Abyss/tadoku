<script lang="ts">
  interface Props {
    text?: string;
    image?: string | null;
    style?: string;
    tooltip: string;
    children?: any;
    onclick: (event: MouseEvent) => void;
  }

  let {
    text = '',
    image = null,
    style = '',
    tooltip = '',
    children,
    onclick = () => {},
  }: Props = $props();
</script>

<div class="sidebar-button-wrapper" {style}>
  <span id="border"></span>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div id="btn" {onclick}>
    <div id="btn__content">
      {#if image}
        <img src={image} alt="game-icon" />
      {:else if text}
        {text}
      {:else}
        {@render children?.()}
      {/if}
    </div>
  </div>
  {#if tooltip}
    <span class="tooltip" role="tooltip">{tooltip}</span>
  {/if}
</div>

<style>
  .sidebar-button-wrapper {
    width: 100%;
    cursor: pointer;
    display: flex;
    align-items: center;
    position: relative;

    &:hover #border {
      opacity: 1;
    }

    &:hover .tooltip {
      opacity: 1;
      transition-delay: 200ms;
    }
  }

  #btn {
    width: 56px;
    height: 56px;
    border: 0;
    color: var(--main-text);
    overflow: hidden;
    background: rgba(185, 154, 250, 0.17);
    border-radius: var(--big-radius);
    box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border: 1px solid rgba(185, 154, 250, 0.13);
  }

  #btn__content {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    & img {
      height: 50%;
      width: 50%;
      border-radius: 5px;
    }
  }

  #border {
    height: 28px;
    width: 3px;
    opacity: 0;
    border-radius: 0px 8px 8px 0px;
    background-color: var(--secondary);
    margin-right: 10px;
    transition: opacity 0.2s ease-in-out;
  }

  .tooltip {
    position: absolute;
    left: calc(100% + 12px);
    top: 50%;
    transform: translateY(-50%);
    background: var(--accent);
    color: var(--main-text);
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    white-space: nowrap;
    z-index: 9999;
    pointer-events: none;
    user-select: none;
    opacity: 0;
    transition: opacity 0.2s ease;
    box-shadow:
      0 4px 12px rgba(0, 0, 0, 0.4),
      0 2px 4px rgba(0, 0, 0, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .tooltip::before {
    content: '';
    position: absolute;
    left: -6px;
    top: 50%;
    transform: translateY(-50%);
    width: 0;
    height: 0;
    border-top: 6px solid transparent;
    border-bottom: 6px solid transparent;
    border-right: 6px solid var(--accent);
  }
</style>
