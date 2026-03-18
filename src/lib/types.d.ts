export interface Novel extends Game {
  id: string;
}

type ColorSwatch = {
  color: string;
  index: number;
};

export type Fetchable<T> =
  | { type: 'notFetched' }
  | { type: 'notFound' }
  | { type: 'available'; value: T };

export interface GameDto {
  title: string;
  description: string;
  /** Is a local file path when loading games only, otherwise it's VNDB image URL.*/
  image_url: string;
  exe_file_path: string;
  process_file_path: string;
  is_nsfw: boolean;
  characters: Character[] | null;
  alt_title: string | null;
}

export interface Character {
  id: string;
  en_name: string;
  og_name: string | null;
  image_url: string | null;
}

export interface Game extends GameDto {
  alt_title: Fetchable<string>;
  /** Play time in seconds. */
  playtime: number;
  today_playtime: number;
  last_play_date: string | null;
  last_played: number | null;
  first_played: number | null;
  is_pinned: boolean;
  categories: string[];
  notes: string;
  icon_url: string | null;
  /** Cumulative characters read (from exSTATic) */
  chars_read: number;
  /** Total character count from Jiten API (pre-fetched at startup) */
  jiten_char_count: Fetchable<number>;
}

export interface Options {
  include_characters: boolean;
}

export interface CurrentGame {
  id: string;
}

export interface ProcessItem {
  title: string;
  exe_path: string;
  icon: string | null;
}

export interface ThemeSettings {
  /** The theme ID (default, tokyo-night, etc) */
  theme: string;
  /** The custom accent color */
  accentColor: string;
  /** Whether to use custom color */
  useCustomColor: boolean;
}

export interface Theme {
  id: string;
  name: string;
  primary: string;
  accent: string;
  background: string;
}

export interface VndbResult {
  id: string;
  title: string;
  alttitle: string | null;
  image: {
    url: string;
    sexual: number;
  };
  description: string | null;
}

export type SortOrder = 'title' | 'last_played' | 'playtime';

export type DiscordPresenceMode = 'All' | 'None' | 'InGame';

export type PlaytimeMode = 'classic' | 'ex_static';

export type Tab = {
  id: string;
  label: string;
  visible: boolean;
  disabled: boolean;
  loading: boolean;
};
