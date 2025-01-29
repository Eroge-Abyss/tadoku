export interface Novel extends Game {
  id: string;
}

interface Game {
  title: string;
  description: string;
  image_url: string; // Is a local file path when loading games only, otherwise it's VNDB image URL.
  exe_file_path: string;
  playtime: number; // Play time in seconds.
  is_pinned: boolean
}

export interface AppState {
  gamesList: Record<string, Game>;
}
