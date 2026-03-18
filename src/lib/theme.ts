import { THEMES } from './constants';
import type { ThemeSettings } from './types';

export function applyTheme(settings: ThemeSettings): void {
  const theme = THEMES.find((t) => t.id === settings.theme) || THEMES[0];

  document.documentElement.style.setProperty(
    '--primary',
    settings.useCustomColor ? settings.accentColor : theme.primary,
  );
  document.documentElement.style.setProperty(
    '--main-background',
    theme.background,
  );
  document.documentElement.style.setProperty('--accent', theme.accent);
  document.documentElement.style.setProperty('--main-text', '#ffffff');
  document.documentElement.style.setProperty('--secondary-text', '#9ca3af');
  document.documentElement.style.setProperty(
    '--secondary', // Note: This might be a specific theme variable, confirm usage.
    settings.useCustomColor ? settings.accentColor : theme.primary,
  );

  document.documentElement.setAttribute('data-theme', settings.theme);
}
