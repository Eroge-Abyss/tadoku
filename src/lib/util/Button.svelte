<script>
    /**
     * @typedef {Object} Props
     * @property {string} [text]
     * @property {string|null} [image]
     * @property {string|null} [icon]
     * @property {string} [style]
     * @property {any} [children]
     * @property {(event: MouseEvent) => void} [onclick]
     */

    /** @type {Props} */
    let {
        text = "",
        image = null,
        icon = null,
        style = "",
        children,
        onclick = () => {},
    } = $props();
    console.log(children);
</script>

<div {style}>
    <span id="border"></span>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div id="btn" {onclick}>
        <div id="btn__content">
            {#if image}
                <img src={image} height="56px" width="56px" alt="game-icon" />
            {:else if text}
                {text}
            {:else}
                {@render children?.()}
            {/if}
        </div>
    </div>
</div>

<style>
    div {
        width: 100%;
        cursor: pointer;
        display: flex;
        align-items: center;
        &:hover #border {
            opacity: 1;
        }
        & #btn {
            width: 56px;
            height: 56px;
            border: 0;
            color: var(--main-text);
            overflow: hidden;
            /* From https://css.glass */
            background: rgba(185, 154, 250, 0.17);
            border-radius: 12px;
            box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
            backdrop-filter: blur(20px);
            -webkit-backdrop-filter: blur(20px);
            border: 1px solid rgba(185, 154, 250, 0.13);
            & #btn__content {
                width: 100%;
                height: 100%;
                display: flex;
                justify-content: center;
                align-items: center;
            }
        }
        & #border {
            height: 28px;
            width: 3px;
            opacity: 0;
            border-radius: 0px 8px 8px 0px;
            background-color: var(--main-mauve);
            margin-right: 10px;
            transition: opacity 0.2s ease-in-out;
        }
    }
</style>
