<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { appState } from '$lib/state.svelte';
  import { resolve } from '$app/paths';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { goto } from '$app/navigation';

  let sessionTime = $state(0);
  let isPaused = $state(false);
  let unlisten: UnlistenFn | undefined;
  let inactivityTimeout: ReturnType<typeof setTimeout> | undefined;

  const resetInactivityTimeout = () => {
    if (inactivityTimeout) clearTimeout(inactivityTimeout);
    inactivityTimeout = setTimeout(() => {
      isPaused = true;
    }, 60_000); // Pause if no events received for 60 seconds
  };

  onMount(async () => {
    unlisten = await listen<{ status: 'playing' | 'paused'; time: number }>(
      'playtime',
      (event) => {
        sessionTime = event.payload.time;
        isPaused = event.payload.status === 'paused';

        if (!isPaused) {
          resetInactivityTimeout();
        } else if (inactivityTimeout) {
          clearTimeout(inactivityTimeout);
        }
      },
    );
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    if (inactivityTimeout) clearTimeout(inactivityTimeout);
  });

  const currentGameData = $derived.by(() => {
    const currentId = appState.currentGame?.id;
    if (!currentId) return null;
    return appState.gamesList[currentId] ?? null;
  });

  const formatDuration = (seconds: number): string => {
    const safe = Math.max(0, Math.floor(seconds));
    const h = Math.floor(safe / 3600);
    const m = Math.floor((safe % 3600) / 60);
    const s = safe % 60;

    return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s
      .toString()
      .padStart(2, '0')}`;
  };

  const openCurrentGame = () => {
    const id = appState.currentGame?.id;
    if (!id) return;
    goto(resolve(`/novel/${id}`));
  };
</script>

{#if currentGameData}
  <button
    class="now-playing"
    onclick={openCurrentGame}
    aria-label="open-currently-playing-game"
    title="Open currently playing game"
  >
    <div class="status-indicator">
      <span class="status-dot" class:paused={isPaused}></span>
    </div>
    <span class="title">{currentGameData.title}</span>
    <span class="divider"></span>
    <span class="counter">{formatDuration(sessionTime)}</span>
  </button>
{/if}

<style>
  .now-playing {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    background-color: color-mix(in srgb, var(--accent) 10%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent) 30%, transparent);
    border-radius: var(--big-radius, 8px);
    padding: 0.4rem 0.8rem 0.4rem 0.6rem;
    color: var(--main-text);
    font-family: inherit;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    max-width: 300px;
  }

  .now-playing:hover {
    background-color: color-mix(in srgb, var(--accent) 15%, transparent);
    border-color: color-mix(in srgb, var(--accent) 50%, transparent);
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: var(--secondary);
    transition: opacity 0.3s ease;
  }

  .status-dot.paused {
    opacity: 0.3;
  }

  .title {
    font-size: 0.85rem;
    font-weight: 600;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
    max-width: 150px;
  }

  .divider {
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background-color: var(--secondary);
    opacity: 0.5;
  }

  .counter {
    font-size: 0.85rem;
    font-weight: 700;
    color: var(--secondary);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }
</style>
