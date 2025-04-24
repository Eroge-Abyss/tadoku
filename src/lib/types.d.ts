export interface Novel extends Game {
  id: string;
}

export interface Game {
  title: string;
  description: string;
  /** Is a local file path when loading games only, otherwise it's VNDB image URL.*/
  image_url: string;
  exe_file_path: string;
  process_file_path: string;
  /** Play time in seconds. */
  playtime: number;
  last_played: number | null;
  is_pinned: boolean;
  is_nsfw: boolean;
  icon_url: string | null;
  categories: string[];
  characters: {
    id: string;
    en_name: string;
    og_name: string | null;
    image_url: string | null;
  }[];
}

export interface Options {
  include_characters: boolean;
}

export interface CurrentGame {
  id: string;
}

export interface Process {
  title: string;
  exe_path: string;
  icon: string;
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
