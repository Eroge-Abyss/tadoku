<script>
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import bbobHTML from '@bbob/html';
  import html5Preset from '@bbob/preset-html5';
  import { fly } from 'svelte/transition';
  import { open } from '@tauri-apps/plugin-dialog';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { platform } from '@tauri-apps/plugin-os';

  let {
    novel,
    playing,
    activeMenu = $bindable(),
    onStartGame,
    onStopGame,
    onTogglePin,
    onEditExe,
    onProcessDialog,
    onDeleteDialog,
  } = $props();

  const customHtml5Preset = html5Preset.extend((tags) => ({
    ...tags,
    url: (node, params) => {
      const tag = tags.url(node, params, {});
      // @ts-ignore
      const href = tag.attrs?.href.startsWith('/')
        ? `https://vndb.org${tag.attrs?.href}`
        : tag.attrs?.href;

      return {
        ...tag,
        attrs: {
          ...tag.attrs,
          target: '_blank',
          rel: 'noopener noreferer',
          href,
        },
      };
    },
  }));
</script>

<div class="header">
  <div class="novel-info" in:fly={{ x: 10, duration: 500 }}>
    {#if !novel.is_nsfw}
      <img
        src={convertFileSrc(novel.image_url)}
        alt={novel.title}
        class="novel-image"
        in:fly={{ y: 50, duration: 500, delay: 300 }}
      />
    {:else}
      <img
        src={convertFileSrc(novel.image_url)}
        alt={novel.title}
        class="novel-image blur"
        in:fly={{ y: 50, duration: 500, delay: 300 }}
      />
    {/if}
    <div class="novel-text">
      <h1>{novel.title}</h1>
      <p class="description">
        {@html bbobHTML(novel.description, customHtml5Preset())}
      </p>
    </div>
  </div>
  <div class="buttons">
    {#if playing}
      <button onclick={onStopGame} class="playing">Close</button>
    {:else}
      <button onclick={onStartGame}>Start</button>
    {/if}
    <i
      class="fa-solid fa-ellipsis fa-xl"
      onclick={() => (activeMenu = !activeMenu)}
      class:active={activeMenu}
    ></i>
    <div class="menu" class:active={activeMenu}>
      <i
        onclick={onTogglePin}
        class={[
          'fa-solid',
          novel.is_pinned ? 'fa-thumbtack-slash' : 'fa-thumbtack',
        ]}
        title="Toggle pinned"
      ></i>
      <i
        onclick={onEditExe}
        class="fa-regular fa-pen-to-square"
        title="Edit exe path"
      ></i>
      <i
        onclick={onProcessDialog}
        class="fa-solid fa-folder-tree"
        title="Change game process path"
      ></i>
      <i
        onclick={() => openUrl(`https://vndb.org/${novel.id}`)}
        class="fa-solid fa-arrow-up-right-from-square"
        title="Open in VNDB"
      ></i>
      <i
        onclick={() => invoke('set_characters', { gameId: novel.id })}
        class="fa-solid fa-user-plus"
        title="Download characters list"
      ></i>
      <i
        onclick={onDeleteDialog}
        class="fa-regular fa-trash-can"
        style="color:  #f7768e;"
        title="Delete game"
      ></i>
    </div>
  </div>
</div>

<style>
  .blur {
    filter: blur(5px);
    transition: filter 0.2s ease-in-out;
  }

  .blur:hover {
    filter: blur(0);
  }

  .header {
    display: flex;
    flex-direction: column;
    padding: 2rem;
    gap: 1rem;
  }

  .novel-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .novel-text {
    height: 100%;
  }

  .novel-image {
    width: 150px;
    height: 200px;
    object-fit: cover;
    border-radius: 8px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
  }

  h1 {
    color: var(--foreground);
    margin: 0 0 0.5rem 0;
    font-size: 2.5rem;
    font-weight: 700;
  }

  .description {
    flex: 1;
    color: var(--main-text);
    font-size: 14px;
    font-weight: 600;
    line-height: 1.5;
    overflow-x: hidden;
    overflow-y: scroll;
    max-height: 200px;
  }

  .buttons,
  .buttons .menu {
    display: flex;
    gap: 1rem;
    align-items: center;
    margin-top: 1rem;

    & .menu {
      margin-top: 0;
      opacity: 0;
      transform: translateX(-20px);
      transition:
        opacity 0.3s ease,
        transform 0.3s ease;
      pointer-events: none;
      & i {
        opacity: 0;
        transform: translateY(-10px);
        transition:
          opacity 0.3s ease,
          transform 0.3s ease;
        &:hover {
          color: var(--main-text);
        }
      }
      &.active {
        transform: translateX(0);
        z-index: 0;
        opacity: 1;
        & i {
          opacity: 1;
          transform: translateY(0);
          pointer-events: auto;
          &:nth-child(1) {
            transition-delay: 0.1s;
          }
          &:nth-child(2) {
            transition-delay: 0.2s;
          }
          &:nth-child(3) {
            transition-delay: 0.3s;
          }
          &:nth-child(4) {
            transition-delay: 0.4s;
          }
          &:nth-child(5) {
            transition-delay: 0.5s;
          }
          &:nth-child(6) {
            transition-delay: 0.6s;
          }
        }
      }
    }
    i {
      color: #464545;
      transition: all 0.2s ease-in-out;
      cursor: pointer;
      &.active {
        color: var(--main-text);
        transform: rotate(90deg);
      }
    }

    & button {
      border: 0;
      border-radius: var(--small-radius);
      color: var(--main-text);
      background-color: #313131;
      width: 200px;
      padding: 0.5rem;
      font-size: 18px;
      cursor: pointer;
      transition: background-color 0.3s ease;

      &:first-child {
        background: var(--primary);

        &:hover {
          background: var(
            --primary-dark,
            color-mix(in srgb, var(--primary), #000 10%)
          );
        }

        &.playing {
          background: rgb(224, 90, 90);
          &:hover {
            background: color-mix(in srgb, var(--primary), rgb(244, 65, 98));
          }
        }
      }
    }
  }
</style>
