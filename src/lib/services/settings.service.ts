import type {
  DiscordPresenceMode,
  PlaytimeMode,
  SortOrder,
  ThemeSettings,
} from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

export async function getSortOrder(): Promise<SortOrder> {
  return invoke('get_sort_order');
}

export async function setSortOrder(sortOrder: SortOrder): Promise<void> {
  await invoke('set_sort_order', { sortOrder });
}

export async function getTheme(): Promise<ThemeSettings> {
  const { theme, accent_color, use_custom_accent } = await invoke<{
    theme: string;
    accent_color: string;
    use_custom_accent: boolean;
  }>('get_theme_settings');

  return {
    theme,
    accentColor: accent_color,
    useCustomColor: use_custom_accent,
  };
}

export async function setTheme(settings: ThemeSettings): Promise<void> {
  await invoke('set_theme_settings', {
    themeSettings: {
      accent_color: settings.accentColor,
      theme: settings.theme,
      use_custom_accent: settings.useCustomColor,
    },
  });
}

export async function getDiscordMode(): Promise<DiscordPresenceMode> {
  return invoke('get_discord_presence_mode');
}

export async function setDiscordMode(mode: DiscordPresenceMode): Promise<void> {
  await invoke('set_discord_presence_mode', { to: mode });
}

export async function getDisablePresenceOnNsfw(): Promise<boolean> {
  return invoke('get_nsfw_presence_status');
}

export async function setDisablePresenceOnNsfw(to: boolean): Promise<void> {
  await invoke('set_nsfw_presence_status', { to });
}

export async function getPlaytimeMode(): Promise<PlaytimeMode> {
  return invoke('get_playtime_mode');
}

export async function setPlaytimeMode(mode: PlaytimeMode): Promise<void> {
  await invoke('set_playtime_mode', { to: mode });
}

export async function getShowRandomButton(): Promise<boolean> {
  return invoke('get_show_random_picker');
}
export async function setShowRandomButton(to: boolean): Promise<void> {
  await invoke('set_show_random_picker', { to });
}

export async function getUseJpForTitleTime(): Promise<boolean> {
  return invoke('get_use_jp_for_title_time');
}
export async function setUseJpForTitleTime(to: boolean): Promise<void> {
  await invoke('set_use_jp_for_title_time', { to });
}

export async function getHideNsfwImages(): Promise<boolean> {
  return invoke('get_hide_nsfw_images');
}
export async function setHideNsfwImages(to: boolean): Promise<void> {
  await invoke('set_hide_nsfw_images', { to });
}

export async function getSelectedCategories(): Promise<string[]> {
  return invoke('get_selected_categories');
}
export async function setSelectedCategories(
  categories: string[],
): Promise<void> {
  invoke('set_selected_categories', { categories });
}
