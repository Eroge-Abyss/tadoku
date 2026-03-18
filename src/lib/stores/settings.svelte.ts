import { DEFAULT_THEME_SETTINGS } from '$lib/constants';
import * as settingsService from '$lib/services/settings.service';
import { applyTheme } from '$lib/theme';
import { toast } from 'svelte-sonner';
import type {
  DiscordPresenceMode,
  PlaytimeMode,
  SortOrder,
  ThemeSettings,
} from '$lib/types';

class SettingsStore {
  #theme: ThemeSettings = $state({ ...DEFAULT_THEME_SETTINGS });
  #discordMode: DiscordPresenceMode = $state('All');
  #playtimeMode: PlaytimeMode = $state('classic');
  #showRandomButton: boolean = $state(false);
  #useJpForTitleTime: boolean = $state(false);
  #hideNsfwImages: boolean = $state(false);
  #sortOrder: SortOrder | null = $state(null);
  #disablePresenceOnNsfw: boolean = $state(true);
  #selectedCategories: string[] = $state([]);

  async init(): Promise<void> {
    const [
      theme,
      discordMode,
      playtimeMode,
      showRandom,
      useJp,
      hideNsfw,
      sortOrder,
      disablePresenceOnNsfw,
      selectedCategories,
    ] = await Promise.all([
      settingsService.getTheme(),
      settingsService.getDiscordMode(),
      settingsService.getPlaytimeMode(),
      settingsService.getShowRandomButton(),
      settingsService.getUseJpForTitleTime(),
      settingsService.getHideNsfwImages(),
      settingsService.getSortOrder(),
      settingsService.getDisablePresenceOnNsfw(),
      settingsService.getSelectedCategories(),
    ]);

    this.#theme = theme;
    this.#discordMode = discordMode;
    this.#playtimeMode = playtimeMode;
    this.#showRandomButton = showRandom;
    this.#useJpForTitleTime = useJp;
    this.#hideNsfwImages = hideNsfw;
    this.#sortOrder = sortOrder;
    this.#disablePresenceOnNsfw = disablePresenceOnNsfw;
    this.#selectedCategories = selectedCategories;

    applyTheme(this.#theme);
  }

  get themeSettings(): ThemeSettings {
    return this.#theme;
  }
  async updateThemeSettings(settings: Partial<ThemeSettings>): Promise<void> {
    try {
      this.#theme = { ...this.#theme, ...settings };
      await settingsService.setTheme(this.#theme);
      applyTheme(this.#theme);
    } catch (error) {
      console.error('Failed to update theme settings:', error);
      toast.error(`Failed to update theme settings: ${error}`);
      throw error;
    }
  }
  async resetThemeSettings(): Promise<void> {
    try {
      this.#theme = { ...DEFAULT_THEME_SETTINGS };
      await settingsService.setTheme(this.#theme);
      applyTheme(this.#theme);
    } catch (error) {
      console.error('Failed to reset theme settings:', error);
      toast.error(`Failed to reset theme settings: ${error}`);
      throw error;
    }
  }

  get discordPresenceMode(): DiscordPresenceMode {
    return this.#discordMode;
  }
  async setDiscordPresenceMode(mode: DiscordPresenceMode): Promise<void> {
    try {
      this.#discordMode = mode;
      await settingsService.setDiscordMode(mode);
    } catch (error) {
      console.error('Failed to set Discord presence mode:', error);
      toast.error(`Failed to set Discord presence mode: ${error}`);
      throw error;
    }
  }

  get playtimeMode(): PlaytimeMode {
    return this.#playtimeMode;
  }
  async setPlaytimeMode(mode: PlaytimeMode): Promise<void> {
    try {
      this.#playtimeMode = mode;
      await settingsService.setPlaytimeMode(mode);
    } catch (error) {
      console.error('Failed to set playtime mode:', error);
      toast.error(`Failed to set playtime mode: ${error}`);
      throw error;
    }
  }

  get showRandomButton(): boolean {
    return this.#showRandomButton;
  }
  async setShowRandomButton(show: boolean): Promise<void> {
    try {
      this.#showRandomButton = show;
      await settingsService.setShowRandomButton(show);
    } catch (error) {
      console.error('Failed to set show random button:', error);
      toast.error(`Failed to set show random button: ${error}`);
      throw error;
    }
  }

  get useJpForTitleTime(): boolean {
    return this.#useJpForTitleTime;
  }
  async setUseJpForTitleTime(useJp: boolean): Promise<void> {
    try {
      this.#useJpForTitleTime = useJp;
      await settingsService.setUseJpForTitleTime(useJp);
    } catch (error) {
      console.error('Failed to set use JP for title time:', error);
      toast.error(`Failed to set use JP for title time: ${error}`);
      throw error;
    }
  }

  get hideNsfwImages(): boolean {
    return this.#hideNsfwImages;
  }
  async setHideNsfwImages(hide: boolean): Promise<void> {
    try {
      this.#hideNsfwImages = hide;
      await settingsService.setHideNsfwImages(hide);
    } catch (error) {
      console.error('Failed to set hide NSFW images:', error);
      toast.error(`Failed to set hide NSFW images: ${error}`);
      throw error;
    }
  }

  get sortOrder(): SortOrder | null {
    return this.#sortOrder;
  }
  async setSortOrder(order: SortOrder): Promise<void> {
    try {
      this.#sortOrder = order;
      await settingsService.setSortOrder(order);
    } catch (error) {
      console.error('Failed to set sort order:', error);
      toast.error(`Failed to set sort order: ${error}`);
      throw error;
    }
  }

  get disablePresenceOnNsfw(): boolean {
    return this.#disablePresenceOnNsfw;
  }
  async setDisablePresenceOnNsfw(disable: boolean): Promise<void> {
    try {
      this.#disablePresenceOnNsfw = disable;
      await settingsService.setDisablePresenceOnNsfw(disable);
    } catch (error) {
      console.error('Failed to set disable presence on NSFW:', error);
      toast.error(`Failed to set disable presence on NSFW: ${error}`);
      throw error;
    }
  }

  get selectedCategories(): string[] {
    return this.#selectedCategories;
  }
  async setSelectedCategories(categories: string[]): Promise<void> {
    try {
      this.#selectedCategories = categories;
      await settingsService.setSelectedCategories(categories);
    } catch (error) {
      console.error('Failed to set selected categories:', error);
      toast.error(`Failed to set selected categories: ${error}`);
      throw error;
    }
  }
}

export const settingsStore = new SettingsStore();
