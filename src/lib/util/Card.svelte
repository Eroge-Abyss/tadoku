<script>
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  const { id, title, image, playtime, isNsfw } = $props();

  let hoursPlayed = $derived(Math.floor(playtime / 3600));
  let minutesPlayed = $derived(Math.floor((playtime % 3600) / 60));

  let image_url = $derived(convertFileSrc(image));
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<section onclick={() => goto(`/novel/${id}`)} class="card">
  <div class="card-image">
    {#if isNsfw}
      <img src={image_url} alt={title} class="blur" />
    {:else}
      <img src={image_url} alt={title} />
    {/if}
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
      <p class="time">{hoursPlayed}時{minutesPlayed}分</p>
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
    background-color: var(--accent);
    border-radius: 12px;
    overflow: hidden;
    transition:
      transform 0.3s ease,
      box-shadow 0.3s ease,
      background-color 0.3s ease;
    display: block;
    text-decoration: none;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
    max-width: 250px;
    cursor: pointer;
    position: relative;
    will-change: transform, box-shadow;
  }

  .card:hover {
    transform: translateY(-5px);
    box-shadow: 0 12px 20px rgba(0, 0, 0, 0.4);
  }

  .card:hover .card-content h3 {
    color: var(--main-text);
  }

  /* Card glossy overlay effect */
  .card::before {
    content: "";
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.05) 0%,
      rgba(255, 255, 255, 0.02) 40%,
      rgba(255, 255, 255, 0) 60%
    );
    transform: rotate(30deg);
    pointer-events: none;
    z-index: 2;
    opacity: 0;
    transition: opacity 0.5s ease;
  }

  .card:hover::before {
    opacity: 1;
  }

  .card-image {
    position: relative;
    overflow: hidden;
  }
  .card-image::after {
    content: "";
    position: absolute;
    top: -50%;
    left: -100%;
    width: 120%;
    height: 200%;
    background: linear-gradient(
      90deg,
      transparent 0%,
      rgba(255, 255, 255, 0) 10%,
      rgba(255, 255, 255, 0.2) 50%,
      rgba(255, 255, 255, 0) 90%,
      transparent 100%
    );
    transform: rotate(25deg);
    opacity: 0;
    transition: left 0.8s cubic-bezier(0.23, 1, 0.32, 1), opacity 0.6s ease;
    will-change: left, opacity;
    z-index: 2;
    pointer-events: none;
  }

  .card:hover .card-image::after {
    left: 100%;
    opacity: 1;
  }

  .card-image img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s ease-out;
    will-change: transform;
  }

  .card:hover .card-image img {
    transform: scale(1.05);
  }

  .card-content {
    padding: 1rem;
    transition: background-color 0.3s ease;
    position: relative;
    z-index: 1;
  }

  .card-content h3 {
    color: var(--secondary-text);
    margin: 0 0 0.75rem 0;
    font-size: 16px;
    font-weight: 400;
    transition: color 0.2s ease-in-out;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.5; 
    min-height: calc(1.5em * 2); 
  }

  .progress-info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .progress-info .time {
    color: var(--main-mauve);
    font-weight: bold;
    transition: color 0.3s ease;
  }
</style>
