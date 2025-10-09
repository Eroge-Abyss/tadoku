<script>
  import { fly } from 'svelte/transition';

  let {
    hoursPlayed,
    minutesPlayed,
    todayHoursPlayed,
    todayMinutesPlayed,
    firstPlayedDate,
    lastPlayedDate,
    formatRelativeDate,
  } = $props();
</script>

<div class="stats-grid">
  <div class="stat-item" in:fly={{ y: 20, duration: 500 }}>
    <p class="stat-label">Time Played</p>
    <span class="stat-value">{hoursPlayed}h {minutesPlayed}m</span>
  </div>
  <div class="stat-item" in:fly={{ y: 20, duration: 500 }}>
    <p class="stat-label">Time Played Today</p>
    <span class="stat-value">{todayHoursPlayed}h {todayMinutesPlayed}m</span>
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
</style>
