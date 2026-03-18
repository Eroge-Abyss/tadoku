use crate::prelude::Result;
use tauri::AppHandle;

mod background;
mod discord;
mod images;
mod state;
mod store;

pub fn run(app: &AppHandle) -> Result<()> {
    store::migrate(app)?;
    images::ensure_folder(app)?;
    state::initialize(app)?;
    let _ = discord::initialize(app); // intentionally infallible at the top level
    background::spawn(app);
    Ok(())
}
