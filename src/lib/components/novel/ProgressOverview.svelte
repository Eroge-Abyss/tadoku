<script lang="ts">
  import { formatTime } from '$lib/util';
  import { fly } from 'svelte/transition';
  import type { PlaytimeMode } from '$lib/types';

  type Props = {
    hoursPlayed: number;
    minutesPlayed: number;
    todayHoursPlayed: number;
    todayMinutesPlayed: number;
    firstPlayedDate: Date | null;
    lastPlayedDate: Date | null;
    formatRelativeDate: (date: Date) => string;
    jitenCharCount?: number | null;
    charsRead?: number;
    playtimeMode?: PlaytimeMode;
  };

  let {
    hoursPlayed,
    minutesPlayed,
    todayHoursPlayed,
    todayMinutesPlayed,
    firstPlayedDate,
    lastPlayedDate,
    formatRelativeDate,
    jitenCharCount = null,
    charsRead = 0,
    playtimeMode = 'classic',
  }: Props = $props();

  const progressPercent = $derived(
    jitenCharCount !== null && jitenCharCount > 0
      ? Math.min((charsRead / jitenCharCount) * 100, 100)
      : 0,
  );
</script>

<div class="stats-grid">
  <div class="stat-item" in:fly={{ y: 20, duration: 500 }}>
    <p class="stat-label">Time Played</p>
    <span class="stat-value">{formatTime(hoursPlayed, minutesPlayed)}</span>
  </div>
  <div class="stat-item" in:fly={{ y: 20, duration: 500 }}>
    <p class="stat-label">Time Played Today</p>
    <span class="stat-value"
      >{formatTime(todayHoursPlayed, todayMinutesPlayed)}</span
    >
  </div>
  <div class="stat-item" in:fly={{ y: 20, duration: 500 }}>
    <p class="stat-label">First Played</p>
    <span class="stat-value">
      {#if firstPlayedDate}
        {formatRelativeDate(firstPlayedDate)}
      {:else}
        Never Played
      {/if}
    </span>
  </div>
  <div class="stat-item" in:fly={{ y: 20, duration: 500 }}>
    <p class="stat-label">Last Played</p>
    <span class="stat-value">
      {#if lastPlayedDate}
        {formatRelativeDate(lastPlayedDate)}
      {:else}
        Never Played
      {/if}
    </span>
  </div>
  {#if playtimeMode === 'ex_static'}
    <div
      class="stat-item has-progress"
      in:fly={{ y: 20, duration: 500, delay: 100 }}
    >
      <p class="stat-label">Chars Read</p>
      <span class="stat-value">{charsRead.toLocaleString()}</span>
      {#if jitenCharCount !== null && charsRead > 0}
        <span class="progress-badge">{progressPercent.toFixed(1)}%</span>
        <div class="bottom-progress">
          <div
            class="bottom-progress-fill"
            style="width: {progressPercent}%;"
          ></div>
        </div>
      {/if}
    </div>
    <div class="stat-item" in:fly={{ y: 20, duration: 500, delay: 100 }}>
      <p class="stat-label">Total Chars (Jiten)</p>
      <span class="stat-value">
        {#if jitenCharCount !== null}
          {jitenCharCount.toLocaleString()}
        {:else}
          <span class="na-text">N/A</span>
        {/if}
      </span>
    </div>
  {/if}
</div>

<style>
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .stat-item {
    background-color: var(--accent);
    padding: 1rem;
    border-radius: 8px;
    position: relative;
    overflow: hidden;
  }

  .stat-item:hover {
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
  }

  .stat-label {
    color: var(--foreground);
    font-size: 0.875rem;
    margin-bottom: 0.25rem;
    opacity: 0.7;
  }

  .stat-value {
    font-size: 1.5rem;
    font-weight: 600;
  }

  .na-text {
    font-size: 1rem;
    opacity: 0.4;
  }

  .progress-badge {
    position: absolute;
    bottom: 0.5rem;
    right: 0.5rem;
    font-size: 1rem;
    font-weight: 600;
    opacity: 0;
    transition: opacity 0.2s ease;
    color: var(--secondary-text);
    pointer-events: none;
  }

  .has-progress:hover .progress-badge {
    opacity: 1;
  }

  .has-progress {
    padding-bottom: calc(1rem + 4px);
  }

  .bottom-progress {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 4px;
    background-color: rgba(255, 255, 255, 0.08);
  }

  .bottom-progress-fill {
    height: 100%;
    border-radius: 0 2px 2px 0;
    background: var(--secondary);
    transition:
      width 0.5s ease,
      background 0.5s ease;
  }
</style>
