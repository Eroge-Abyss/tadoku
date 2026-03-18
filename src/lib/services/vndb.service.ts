import type { VndbResult } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

export async function search(query: string): Promise<VndbResult[]> {
  return invoke('fetch_vn_info', { key: query });
}
