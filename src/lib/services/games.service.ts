import type { Game, GameDto, ProcessItem, Options } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

export async function load(): Promise<Record<string, Game>> {
  return invoke('load_games');
}
export async function save(
  gameId: string,
  game: GameDto,
  options: Options,
): Promise<void> {
  await invoke('save_game', {
    gameId,
    game,
    options,
  });
}

export async function remove(gameId: string): Promise<void> {
  await invoke('delete_game', { gameId });
}

export async function togglePin(gameId: string): Promise<void> {
  await invoke('toggle_pin', { gameId });
}

export async function resetStats(gameId: string): Promise<void> {
  await invoke('reset_stats', { gameId });
}

export async function setCharacters(gameId: string): Promise<void> {
  await invoke('set_characters', { gameId });
}

export async function updateExePath(
  gameId: string,
  newExePath: string,
): Promise<void> {
  await invoke('update_exe', { gameId, newExePath });
}

export async function updateProcessPath(
  gameId: string,
  newProcessPath: string,
): Promise<void> {
  await invoke('update_process', {
    gameId,
    newProcessPath: newProcessPath,
  });
}

export async function setCategories(
  gameId: string,
  categories: string[],
): Promise<void> {
  await invoke('set_game_categories', { gameId, categories });
}

export async function setNotes(gameId: string, notes: string): Promise<void> {
  await invoke('set_game_notes', { gameId, notes });
}

export async function openGame(gameId: string): Promise<void> {
  await invoke('open_game', { gameId });
}

export async function closeGame(): Promise<void> {
  await invoke('close_game', {});
}

export async function getActiveWindows(): Promise<ProcessItem[]> {
  return await invoke('get_active_windows');
}
