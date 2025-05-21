/**
 * @typedef {import('$lib/types').Theme} Theme
 *  @typedef {import('$lib/types').ThemeSettings} ThemeSettings
 */

/**
 * Theme options
 * @type {Array<Theme>}
 */
export const THEMES = [
  {
    id: 'default',
    name: 'Default',
    primary: '#bb9af7',
    background: '#1b1b1b',
    accent: '#2a2a2a',
  },
  {
    id: 'tokyo-night',
    name: 'Tokyo Night',
    primary: '#bb9af7',
    background: '#1a1b26',
    accent: '#24283b',
  },
  {
    id: 'sakura',
    name: 'Sakura Pink',
    primary: '#f978b6',
    background: '#1f1d2e',
    accent: '#2d2a3e',
  },
  {
    id: 'ocean-blue',
    name: 'Ocean Blue',
    primary: '#7aa2f7',
    background: '#192330',
    accent: '#24283b',
  },
  {
    id: 'forest-green',
    name: 'Forest Green',
    primary: '#9ece6a',
    background: '#1e2030',
    accent: '#282e44',
  },
];

/**
 * Color swatches for custom accent colors
 * @type {string[]}
 */
export const COLOR_SWATCHES = [
  '#3b82f6', // Blue
  '#bb9af7', // Purple
  '#f978b6', // Pink
  '#9ece6a', // Green
  '#7aa2f7', // Light blue
  '#e0af68', // Orange
  '#f7768e', // Red
  '#ffae00', // Yellow
];

/**
 * Default theme settings for application
 * @type {ThemeSettings}
 */
export const DEFAULT_THEME_SETTINGS = {
  theme: 'default',
  accentColor: '#2a2a2a',
  useCustomColor: false,
};
