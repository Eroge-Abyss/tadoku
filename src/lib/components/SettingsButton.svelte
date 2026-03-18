<script lang="ts">
  import { resolve } from '$app/navigation';
  import SidebarButton from '$lib/components/SidebarButton.svelte';

  function handleClick() {
    resolve('/settings');
  }
</script>

<SidebarButton onclick={handleClick} tooltip="Settings">
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
