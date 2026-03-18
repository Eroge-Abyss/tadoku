import type { Novel } from '$lib/types';

export function usePlaytimeStats(getNovel: () => Novel) {
  const novel = $derived(getNovel());

  const hoursPlayed = $derived(Math.floor(novel.playtime / 3600));
  const minutesPlayed = $derived(Math.floor((novel.playtime % 3600) / 60));

  const todayDate = $derived.by(() => {
    const d = new Date();
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
  });

  const todayHoursPlayed = $derived(
    todayDate === novel.last_play_date ? Math.floor((novel.today_playtime || 0) / 3600) : 0
  );

  const todayMinutesPlayed = $derived(
    todayDate === novel.last_play_date ? Math.floor(((novel.today_playtime || 0) % 3600) / 60) : 0
  );

  const lastPlayedDate = $derived(novel.last_played ? new Date(novel.last_played * 1000) : null);
  const firstPlayedDate = $derived(novel.first_played ? new Date(novel.first_played * 1000) : null);

  return {
    get hoursPlayed() { return hoursPlayed; },
    get minutesPlayed() { return minutesPlayed; },
    get todayHoursPlayed() { return todayHoursPlayed; },
    get todayMinutesPlayed() { return todayMinutesPlayed; },
    get lastPlayedDate() { return lastPlayedDate; },
    get firstPlayedDate() { return firstPlayedDate; }
  };
}
