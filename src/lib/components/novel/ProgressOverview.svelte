<script>
  import { formatTime } from '$lib/util';
  import { fly } from 'svelte/transition';

  let {
    hoursPlayed,
    minutesPlayed,
    todayHoursPlayed,
    todayMinutesPlayed,
    firstPlayedDate,
    lastPlayedDate,
    formatRelativeDate,
    jitenCharCount = null,
    jitenLoading = false,
    charsRead = 0,
  } = $props();

  /**
   * Formats a number with thousands separators.
   * @param {number} n
   * @returns {string}
   */
  function formatNumber(n) {
    return n.toLocaleString();
  }
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
  <div class="stat-item" in:fly={{ y: 20, duration: 500, delay: 100 }}>
    <p class="stat-label">Chars Read</p>
    <span class="stat-value">{formatNumber(charsRead)}</span>
  </div>
  <div class="stat-item" in:fly={{ y: 20, duration: 500, delay: 100 }}>
    <p class="stat-label">Total Chars (Jiten)</p>
    <span class="stat-value">
      {#if jitenLoading}
        <span class="loading-text">Loading...</span>
      {:else if jitenCharCount !== null}
        {formatNumber(jitenCharCount)}
      {:else}
        <span class="na-text">N/A</span>
      {/if}
    </span>
  </div>
</div>

{#if !jitenLoading && jitenCharCount !== null && charsRead > 0}
  <div class="progress-bar-container" in:fly={{ y: 20, duration: 500, delay: 200 }}>
    <p class="stat-label">Reading Progress</p>
    <div class="progress-bar-track">
      <div
        class="progress-bar-fill"
        style="width: {Math.min((charsRead / jitenCharCount) * 100, 100)}%"
      ></div>
    </div>
    <span class="progress-text"
      >{Math.min(((charsRead / jitenCharCount) * 100), 100).toFixed(1)}%</span
    >
  </div>
{/if}

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

  .loading-text {
    font-size: 1rem;
    opacity: 0.5;
    font-style: italic;
  }

  .na-text {
    font-size: 1rem;
    opacity: 0.4;
  }

  .progress-bar-container {
    margin-bottom: 2rem;
    padding: 1rem;
    background-color: var(--accent);
    border-radius: 8px;
  }

  .progress-bar-track {
    width: 100%;
    height: 8px;
    background-color: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    overflow: hidden;
    margin: 0.5rem 0;
  }

  .progress-bar-fill {
    height: 100%;
    background: var(--primary);
    border-radius: 4px;
    transition: width 0.5s ease;
  }

  .progress-text {
    font-size: 0.875rem;
    opacity: 0.7;
  }
</style>
