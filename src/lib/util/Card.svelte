<script>
    import { convertFileSrc } from "@tauri-apps/api/core";

    const { id, title, image } = $props();

    let image_url = $derived(convertFileSrc(image));
</script>

<a href={`/novel/${id}`} class="card">
    <div class="card-image">
        <img src={image_url} alt={title} />
        <!-- <span class="category">{category}</span> -->
    </div>
    <div class="card-content">
        <h3>{title}</h3>
        <div class="progress-info">
            <div class="progress-bar">
                <!-- <div
                    class="progress"
                    style="width: {progress.completion}%"
                ></div> -->
            </div>
            <!-- <span class="progress-text">{progress.completion}% Complete</span> -->
        </div>
    </div>
</a>

<style>
    .card {
        background-color: var(--secondary);
        border-radius: 12px;
        overflow: hidden;
        transition:
            transform 0.3s ease,
            box-shadow 0.3s ease;
        display: block;
        text-decoration: none;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    .card:hover {
        transform: translateY(-5px);
        box-shadow: 0 12px 20px rgba(0, 0, 0, 0.2);
    }

    .card-image {
        aspect-ratio: 3/4;
    }

    .card-image img {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }

    .category {
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
    }

    .card-content {
        padding: 1rem;
    }

    .card-content h3 {
        color: var(--main-mauve);
        margin: 0 0 0.75rem 0;
        font-size: 1.25rem;
        font-weight: 600;
    }

    .progress-info {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .progress-bar {
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
    }
</style>
