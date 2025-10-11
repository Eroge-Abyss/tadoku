<script>
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { appState } from '$lib/state.svelte';
  import { formatTime } from '$lib/util';
  const { id, title, image, playtime, isNsfw } = $props();

  const hoursPlayed = $derived(Math.floor(playtime / 3600));
  const minutesPlayed = $derived(Math.floor((playtime % 3600) / 60));

  const image_url = $derived(convertFileSrc(image));
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<section onclick={() => goto(`/novel/${id}`)} class="card">
  <div class="card-image">
    <img src={image_url} alt={title} class:blur={isNsfw} />
    <!-- <span class="category">{category}</span> -->
  </div>
  <div class="card-content">
    <h3>{title}</h3>
    <div class="progress-info">
      <!--div class="progress-bar">
                 <div
                    class="progress"
                    style="width: {progress.completion}%"
                ></div>
            </div-->
      <!-- <span class="progress-text">{progress.completion}% Complete</span> -->
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
  }

  .card-image img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  /* .category {
        position: absolute;
        top: 1rem;
        left: 1rem;
        background-color: rgba(0, 0, 0, 0.7);
        color: var(--main-text);
        padding: 0.5rem 1rem;
        border-radius: 20px;
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    } */

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

  /* .progress-bar {
        width: 100%;
        height: 6px;
        background-color: var(--accent);
        border-radius: 3px;
        overflow: hidden;
    }

    .progress {
        height: 100%;
        background-color: var(--main-background);
        transition: width 0.3s ease;
    }

    .progress-text {
        color: var(--main-text);
        font-size: 0.875rem;
        font-weight: 500;
    } */
</style>
