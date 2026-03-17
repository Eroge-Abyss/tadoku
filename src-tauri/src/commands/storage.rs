use crate::commands::cmd_result::CmdResult;
use crate::services::game_saver::{GameSaver, Options};
use crate::services::state::ManagedState;
use crate::services::{
    discord::DiscordPresenceMode,
    stores::{
        categories::{Categories, CategoriesStore},
        games::{Game, Games, GamesStore},
        settings::{PlaytimeMode, SortOrder, ThemeSettings},
    },
};
use anyhow::Context;
use log::{debug, info};
use tauri::{AppHandle, Manager};

/// Saves a game to the local storage.
///
/// **NOTE**: The image is either downloaded from a remote URL or copied from a
/// local path.  When `image_url` is empty (manual entry with no cover) the
/// image step is skipped entirely.
#[tauri::command]
pub async fn save_game(
    app_handle: AppHandle,
    game_id: String,
    game: Game,
    options: Options,
) -> CmdResult<()> {
    GameSaver::new(&app_handle)
        .save(game_id, game, options)
        .await
        .context("Failed to save game")?;
    Ok(())
}

/// Loads all games from JSON storage
#[tauri::command]
pub fn load_games(app_handle: AppHandle) -> CmdResult<Games> {
    debug!("Loading all games from storage");
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    let games_data = store
        .get_all()
        .context("Error happened while getting games")?;

    debug!(
        "Successfully loaded {} games from storage",
        games_data.len()
    );
    Ok(games_data)
}

/// Deletes a game from JSON storage
#[tauri::command]
pub fn delete_game(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    info!("Deleting game: {}", game_id);
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .delete(&game_id)
        .context("Error happened while deleting game")?;

    info!("Successfully deleted game: {}", game_id);
    Ok(())
}

/// Toggles the pinned state of a game
#[tauri::command]
pub fn toggle_pin(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    debug!("Toggling pin state for game: {}", game_id);
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .update_game(&game_id, |g| g.is_pinned = !g.is_pinned)
        .context("Error happened while toggling pin")?;

    info!("Successfully toggled pin state for game: {}", game_id);
    Ok(())
}

/// Resets game stats
#[tauri::command]
pub fn reset_stats(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    debug!("Resetting stats for game {}", game_id);
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .reset_stats(&game_id)
        .context("Error happened while resetting stats")?;

    info!("Successfully reset stats for game: {}", game_id);
    Ok(())
}

/// Updates the exe path of a game
#[tauri::command]
pub fn update_exe(app_handle: AppHandle, game_id: String, new_exe_path: String) -> CmdResult<()> {
    info!("Updating exe path for game {}: {}", game_id, new_exe_path);
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .update_game(&game_id, |g| g.exe_file_path = new_exe_path)
        .context("Error happened while updating exe")?;

    info!("Successfully updated exe path for game: {}", game_id);
    Ok(())
}

/// Updates the process path of a game
#[tauri::command]
pub fn update_process(
    app_handle: AppHandle,
    game_id: String,
    new_process_path: String,
) -> CmdResult<()> {
    info!(
        "Updating process path for game {}: {}",
        game_id, new_process_path
    );
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .update_game(&game_id, |g| g.process_file_path = new_process_path)
        .context("Error happened while updating process path")?;

    info!("Successfully updated process path for game: {}", game_id);
    Ok(())
}

/// Saves game notes to disk
#[tauri::command]
pub fn set_game_notes(app_handle: AppHandle, game_id: String, notes: String) -> CmdResult<()> {
    debug!(
        "Setting notes for game {}: {} characters",
        game_id,
        notes.len()
    );
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .update_game(&game_id, |g| g.notes = notes)
        .context("Error happened while setting notes")?;

    info!("Successfully set notes for game: {}", game_id);
    Ok(())
}

/// Sets the characters of an already saved game
#[tauri::command]
pub async fn set_characters(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    info!("Setting characters for game: {}", game_id);
    let game_saver = GameSaver::new(&app_handle);
    let characters = game_saver.fetch_characters(&game_id).await?;
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    let characters_len = characters.len();
    store
        .update_game(&game_id, |g| g.characters = Some(characters))
        .context("Error happened while saving characters")?;

    info!(
        "Successfully set {} characters for game: {}",
        characters_len, game_id
    );
    Ok(())
}

/// Gets all categories as an array
#[tauri::command]
pub fn get_categories(app_handle: AppHandle) -> CmdResult<Categories> {
    debug!("Getting all categories");
    let store =
        CategoriesStore::new(&app_handle).context("Error happened while accessing store")?;

    let categories = store.get_all().context("Couldn't get categories")?;

    debug!("Successfully loaded {} categories", categories.len());
    Ok(categories)
}

/// Sets categories from an array
#[tauri::command]
pub fn set_categories(app_handle: AppHandle, categories: Categories) -> CmdResult<()> {
    info!("Setting {} categories", categories.len());
    let store =
        CategoriesStore::new(&app_handle).context("Error happened while accessing store")?;
    let categories_len = categories.len();

    store
        .set(categories)
        .context("Error happened while setting categories")?;

    info!("Successfully set {} categories", categories_len);
    Ok(())
}

/// Gets all categories as an array
#[tauri::command]
pub fn get_selected_categories(app_handle: AppHandle) -> CmdResult<Categories> {
    debug!("Getting all selected categories");
    let store =
        CategoriesStore::new(&app_handle).context("Error happened while accessing store")?;
    let categories = store.get_selected().context("Couldn't get categories")?;

    debug!(
        "Successfully loaded {} selected categories",
        categories.len()
    );
    Ok(categories)
}

/// Sets categories from an array
#[tauri::command]
pub fn set_selected_categories(app_handle: AppHandle, categories: Categories) -> CmdResult<()> {
    info!("Setting {} selected categories", categories.len());
    let store =
        CategoriesStore::new(&app_handle).context("Error happened while accessing store")?;
    let categories_len = categories.len();

    store
        .set_selected(categories)
        .context("Error happened while setting categories")?;

    info!("Successfully set {} selected categories", categories_len);
    Ok(())
}

/// Sets categories of a game from an array
#[tauri::command]
pub fn set_game_categories(
    app_handle: AppHandle,
    game_id: String,
    categories: Categories,
) -> CmdResult<()> {
    info!(
        "Setting {} categories for game: {}",
        categories.len(),
        game_id
    );
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;
    let categories_len = categories.len();

    store
        .update_game(&game_id, |g| g.categories = categories)
        .context("Error happened while setting categories")?;

    info!(
        "Successfully set {} categories for game: {}",
        categories_len, game_id
    );
    Ok(())
}

/// Gets theme settings from storage
#[tauri::command]
pub fn get_theme_settings(app_handle: AppHandle) -> CmdResult<ThemeSettings> {
    Ok(app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .theme_settings
        .clone())
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_theme_settings(app_handle: AppHandle, theme_settings: ThemeSettings) -> CmdResult<()> {
    let state = app_handle.state::<ManagedState>();
    let mut lock = state.lock()?;
    lock.update_settings(&app_handle, |s| s.theme_settings = theme_settings)
        .context("Failed to update theme settings")?;
    Ok(())
}

/// Gets nsfw presence toggle status
#[tauri::command]
pub fn get_nsfw_presence_status(app_handle: AppHandle) -> CmdResult<bool> {
    Ok(app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .disable_presence_on_nsfw)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_nsfw_presence_status(app_handle: AppHandle, to: bool) -> CmdResult<()> {
    let state = app_handle.state::<ManagedState>();
    let mut lock = state.lock()?;
    lock.update_settings(&app_handle, |s| s.disable_presence_on_nsfw = to)
        .context("Failed to update nsfw presence status")?;
    Ok(())
}

/// Gets sort order
#[tauri::command]
pub fn get_sort_order(app_handle: AppHandle) -> CmdResult<SortOrder> {
    Ok(app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .sort_order)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_sort_order(app_handle: AppHandle, sort_order: SortOrder) -> CmdResult<()> {
    let state = app_handle.state::<ManagedState>();
    let mut lock = state.lock()?;
    lock.update_settings(&app_handle, |s| s.sort_order = sort_order)
        .context("Failed to update sort order")?;
    Ok(())
}

/// Gets show random picker
#[tauri::command]
pub fn get_show_random_picker(app_handle: AppHandle) -> CmdResult<bool> {
    Ok(app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .show_random_picker)
}

/// Saves show random picker setting to storage
#[tauri::command]
pub fn set_show_random_picker(app_handle: AppHandle, to: bool) -> CmdResult<()> {
    let state = app_handle.state::<ManagedState>();
    let mut lock = state.lock()?;
    lock.update_settings(&app_handle, |s| s.show_random_picker = to)
        .context("Failed to update show random picker")?;
    Ok(())
}

/// Gets discord presence mode
#[tauri::command]
pub fn get_discord_presence_mode(app_handle: AppHandle) -> CmdResult<DiscordPresenceMode> {
    Ok(app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .discord_presence_mode)
}

/// Saves Discord presence mode setting to storage
#[tauri::command]
pub fn set_discord_presence_mode(app_handle: AppHandle, to: DiscordPresenceMode) -> CmdResult<()> {
    let state = app_handle.state::<ManagedState>();
    let mut lock = state.lock()?;

    lock.update_settings(&app_handle, |s| s.discord_presence_mode = to)
        .context("Failed to update discord presence mode")?;

    if let Some(presence) = lock.presence.as_mut() {
        presence.set_mode(to);
    }

    Ok(())
}

/// Gets playtime mode
#[tauri::command]
pub fn get_playtime_mode(app_handle: AppHandle) -> CmdResult<PlaytimeMode> {
    Ok(app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .playtime_mode)
}

/// Saves new playtime mode to disk
#[tauri::command]
pub fn set_playtime_mode(app_handle: AppHandle, to: PlaytimeMode) -> CmdResult<()> {
    let state = app_handle.state::<ManagedState>();
    let mut lock = state.lock()?;
    lock.update_settings(&app_handle, |s| s.playtime_mode = to)
        .context("Failed to update playtime mode")?;
    Ok(())
}

#[tauri::command]
pub fn get_use_jp_for_title_time(app_handle: AppHandle) -> CmdResult<bool> {
    Ok(app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .use_jp_for_title_time)
}

#[tauri::command]
pub fn set_use_jp_for_title_time(app_handle: AppHandle, to: bool) -> CmdResult<()> {
    let state = app_handle.state::<ManagedState>();
    let mut lock = state.lock()?;
    lock.update_settings(&app_handle, |s| s.use_jp_for_title_time = to)
        .context("Failed to update use jp for title time")?;
    Ok(())
}

#[tauri::command]
pub fn get_hide_nsfw_images(app_handle: AppHandle) -> CmdResult<bool> {
    Ok(app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .hide_nsfw_images)
}

#[tauri::command]
pub fn set_hide_nsfw_images(app_handle: AppHandle, to: bool) -> CmdResult<()> {
    let state = app_handle.state::<ManagedState>();
    let mut lock = state.lock()?;
    lock.update_settings(&app_handle, |s| s.hide_nsfw_images = to)
        .context("Failed to update hide nsfw images")?;
    Ok(())
}

/// Gets the Jiten API base URL
#[tauri::command]
pub fn get_jiten_base_url(app_handle: AppHandle) -> CmdResult<String> {
    Ok(app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .jiten_base_url
        .clone())
}

/// Sets the Jiten API base URL
#[tauri::command]
pub fn set_jiten_base_url(app_handle: AppHandle, url: String) -> CmdResult<()> {
    let state = app_handle.state::<ManagedState>();
    let mut lock = state.lock()?;
    lock.update_settings(&app_handle, |s| s.jiten_base_url = url)
        .context("Failed to update jiten base url")?;
    Ok(())
}
