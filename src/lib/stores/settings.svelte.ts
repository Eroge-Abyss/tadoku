import { DEFAULT_THEME_SETTINGS } from '$lib/constants';
import * as settingsService from '$lib/services/settings.service';
import { applyTheme } from '$lib/theme';
import type { DiscordPresenceMode, PlaytimeMode, SortOrder, ThemeSettings } from '$lib/types';

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

  get themeSettings(): ThemeSettings { return this.#theme; }
  async updateThemeSettings(settings: Partial<ThemeSettings>): Promise<void> {
    this.#theme = { ...this.#theme, ...settings };
    await settingsService.setTheme(this.#theme);
    applyTheme(this.#theme);
  }
  async resetThemeSettings(): Promise<void> {
    this.#theme = { ...DEFAULT_THEME_SETTINGS };
    await settingsService.setTheme(this.#theme);
    applyTheme(this.#theme);
  }

  get discordPresenceMode(): DiscordPresenceMode { return this.#discordMode; }
  async setDiscordPresenceMode(mode: DiscordPresenceMode): Promise<void> {
    this.#discordMode = mode;
    await settingsService.setDiscordMode(mode);
  }

  get playtimeMode(): PlaytimeMode { return this.#playtimeMode; }
  async setPlaytimeMode(mode: PlaytimeMode): Promise<void> {
    this.#playtimeMode = mode;
    await settingsService.setPlaytimeMode(mode);
  }

  get showRandomButton(): boolean { return this.#showRandomButton; }
  async setShowRandomButton(show: boolean): Promise<void> {
    this.#showRandomButton = show;
    await settingsService.setShowRandomButton(show);
  }

  get useJpForTitleTime(): boolean { return this.#useJpForTitleTime; }
  async setUseJpForTitleTime(useJp: boolean): Promise<void> {
    this.#useJpForTitleTime = useJp;
    await settingsService.setUseJpForTitleTime(useJp);
  }

  get hideNsfwImages(): boolean { return this.#hideNsfwImages; }
  async setHideNsfwImages(hide: boolean): Promise<void> {
    this.#hideNsfwImages = hide;
    await settingsService.setHideNsfwImages(hide);
  }

  get sortOrder(): SortOrder | null { return this.#sortOrder; }
  async setSortOrder(order: SortOrder): Promise<void> {
    this.#sortOrder = order;
    await settingsService.setSortOrder(order);
  }

  get disablePresenceOnNsfw(): boolean { return this.#disablePresenceOnNsfw; }
  async setDisablePresenceOnNsfw(disable: boolean): Promise<void> {
    this.#disablePresenceOnNsfw = disable;
    await settingsService.setDisablePresenceOnNsfw(disable);
  }

  get selectedCategories(): string[] { return this.#selectedCategories; }
  async setSelectedCategories(categories: string[]): Promise<void> {
    this.#selectedCategories = categories;
    await settingsService.setSelectedCategories(categories);
  }
}

export const settingsStore = new SettingsStore();
