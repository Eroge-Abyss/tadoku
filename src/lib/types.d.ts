export interface Novel extends Game {
  id: string;
}

export interface Game {
  title: string;
  description: string;
  image_url: string; // Is a local file path when loading games only, otherwise it's VNDB image URL.
  exe_file_path: string;
  process_file_path: string;
  playtime: number; // Play time in seconds.
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
