<script>
  import { goto } from '$app/navigation';
  import SidebarButton from '$lib/components/SidebarButton.svelte';

  function handleClick() {
    goto('/settings');
  }
</script>

<SidebarButton onclick={handleClick}>
  <span class="settings-icon">
    <i class="fas fa-cog"></i>
  </span>
</SidebarButton>

<style>
  .settings-icon {
    font-size: 24px;
    color: #fff;
  }
</style>
