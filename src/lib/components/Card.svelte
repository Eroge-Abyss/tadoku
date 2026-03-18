<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { resolve } from '$app/paths';
  import { formatTime } from '$lib/util';
  import { settingsStore } from '$lib/stores/settings.svelte';
  import NsfwPlaceholder from './NsfwPlaceholder.svelte';
  import { goto } from '$app/navigation';

  type Props = {
    id: string;
    title: string;
    image: string | null;
    playtime: number;
    isNsfw: boolean;
  };
  const { id, title, image, playtime, isNsfw }: Props = $props();

  const hoursPlayed = $derived(Math.floor(playtime / 3600));
  const minutesPlayed = $derived(Math.floor((playtime % 3600) / 60));

  const image_url = $derived(image ? convertFileSrc(image) : '');
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<section onclick={() => goto(resolve(`/novel/${id}`))} class="card">
  <div class="card-image">
    {#if isNsfw && settingsStore.hideNsfwImages}
      <NsfwPlaceholder />
    {:else if image}
      <img src={image_url} alt={title} class:blur={isNsfw} />
    {:else}
      <div class="no-image">
        <i class="fa-solid fa-image"></i>
      </div>
    {/if}
  </div>

  <div class="card-content">
    <h3>{title}</h3>
    <div class="progress-info">
      <p class="time">
        {formatTime(hoursPlayed, minutesPlayed)}
      </p>
    </div>
  </div>
</section>

<style>
  .blur {
    filter: blur(5px);
    transition: filter 0.2s ease-in-out;
  }
  .blur:hover {
    filter: blur(0);
  }
  .card {
    background-color: color-mix(
      in srgb,
      var(--primary) 2.5%,
      var(--main-background) 92%
    );
    color: var(--main-text);
    border-radius: var(--big-radius);
    overflow: hidden;
    transition:
      transform 0.3s ease,
      box-shadow 0.3s ease;
    display: block;
    text-decoration: none;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
    max-width: 250px;
    cursor: pointer;
  }

  .card:hover {
    transform: translateY(-5px);
    box-shadow: 0 12px 20px rgba(0, 0, 0, 0.4);
    & .card-content h3 {
      color: var(--main-text);
    }
  }

  .card-image {
    aspect-ratio: 3/4;
    position: relative;
  }

  .card-image img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .no-image {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--accent);
    color: var(--secondary);
  }

  .no-image i {
    font-size: 3rem;
    opacity: 0.5;
  }

  .card-content {
    padding: 1rem;
  }

  .card-content h3 {
    color: #888;
    margin: 0 0 0.75rem 0;
    font-size: 16px;
    font-weight: 400;
    transition: color 0.2s ease-in-out;
    display: -webkit-box;
    -webkit-line-clamp: 2; /* Number of lines before ellipsis */
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.5; /* Adjust this value if needed */
    min-height: calc(
      1.5em * 2
    ); /* 1.5em is the line height, 2 is the number of lines */
  }

  .progress-info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    & .time {
      color: var(--secondary);
      font-weight: bold;
    }
  }
</style>
