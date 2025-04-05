<script>
    import { goto } from '$app/navigation';
    import Button from '$lib/util/Button.svelte';

    function handleClick() {
        goto('/settings');
    }
</script>

<Button onclick={handleClick}>
    <span class="settings-icon">
        <i class="fas fa-cog"></i>
    </span>
</Button>

<style>
    .settings-icon {
        font-size: 24px;
        color: #fff;
    }
    

</style>