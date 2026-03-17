use crate::prelude::Fetchable;
use crate::services::stores::games::GamesStore;
use crate::services::vndb::{VNDB_MAX_PAGE_SIZE, Vndb};
use log::{error, info, warn};
use tauri::AppHandle;

pub fn spawn(app_handle: &AppHandle) {
    info!("Spawning background task for data fetching");
    fetch_missing_game_data(app_handle);
}

/// Fetches missing alt titles and Jiten character counts for all games in the store.
fn fetch_missing_game_data(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let store = match GamesStore::new(&app_handle) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to create GamesStore for background fetch: {}", e);
                return;
            }
        };

        let all_games = match store.get_all() {
            Ok(games) => games,
            Err(e) => {
                error!("Failed to get games for background fetch: {}", e);
                return;
            }
        };

        let alt_titles_to_fetch: Vec<String> = all_games
            .iter()
            .filter(|(_, game)| game.alt_title == Fetchable::NotFetched)
            .map(|(id, _)| id.clone())
            .collect();

        let jiten_counts_to_fetch: Vec<String> = all_games
            .iter()
            .filter(|(_, game)| game.jiten_char_count == Fetchable::NotFetched)
            .map(|(id, _)| id.clone())
            .collect();

        if alt_titles_to_fetch.is_empty() && jiten_counts_to_fetch.is_empty() {
            info!("Background fetch: all games are up to date.");
            return;
        }

        // Fetch Alt Titles
        let alt_title_results = async {
            if alt_titles_to_fetch.is_empty() {
                return Vec::new();
            }
            info!(
                "Attempting to fetch missing alt titles for {} games",
                alt_titles_to_fetch.len()
            );

            let mut results = Vec::new();
            let total_ids = alt_titles_to_fetch.len();
            let fetch_iterations = total_ids.div_ceil(VNDB_MAX_PAGE_SIZE);

            for i in 0..fetch_iterations {
                let start = i * VNDB_MAX_PAGE_SIZE;
                let end = std::cmp::min(start + VNDB_MAX_PAGE_SIZE, total_ids);
                let ids_slice = &alt_titles_to_fetch[start..end];

                match Vndb::get_vns_alt_title(ids_slice).await {
                    Ok(games_chunk) => {
                        for fetched_game in games_chunk {
                            let state = match fetched_game.alttitle {
                                Some(title) if !title.is_empty() => Fetchable::Available(title),
                                _ => Fetchable::NotFound,
                            };
                            results.push((fetched_game.id, state));
                        }
                    }
                    Err(e) => {
                        warn!("Failed to fetch a chunk of alt titles: {}", e);
                    }
                }
            }
            info!("Alt title fetch completed.");
            results
        }
        .await;

        // Fetch Jiten Counts
        let jiten_results = async {
            use crate::commands::jiten::fetch_jiten_char_count;
            if jiten_counts_to_fetch.is_empty() {
                return Vec::new();
            }
            info!(
                "Attempting to fetch missing Jiten counts for {} games",
                jiten_counts_to_fetch.len()
            );

            let mut results = Vec::new();
            for game_id in &jiten_counts_to_fetch {
                match fetch_jiten_char_count(app_handle.clone(), game_id.clone()).await {
                    Ok(Some(count)) => {
                        results.push((game_id, Fetchable::Available(count)));
                    }
                    Ok(None) => {
                        results.push((game_id, Fetchable::NotFound));
                    }
                    Err(e) => {
                        warn!("Failed to fetch Jiten count for {}: {}", game_id, e);
                    }
                }
            }
            info!("Jiten fetch completed.");
            results
        }
        .await;

        // Apply all updates sequentially
        info!("Applying fetched data to the store.");
        for (id, alt_title) in alt_title_results {
            if let Err(e) = store.update_game(&id, |g| g.alt_title = alt_title) {
                error!("Failed to save alt title for {}: {}", id, e);
            }
        }

        let mut jiten_updated_count = 0;
        let jiten_to_fetch_count = jiten_counts_to_fetch.len();
        for (id, jiten_count) in jiten_results {
            if let Err(e) = store.update_game(id, |g| g.jiten_char_count = jiten_count) {
                error!("Failed to save Jiten count for {}: {}", id, e);
            } else {
                jiten_updated_count += 1;
            }
        }

        if jiten_to_fetch_count > 0 {
            if jiten_updated_count > 0 {
                info!(
                    "Jiten fetch completed: updated {} games",
                    jiten_updated_count
                );
            } else {
                info!("Jiten fetch completed: No new counts found.");
            }
        }

        info!("Background data fetch task completed.");
    });
}
