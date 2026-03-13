use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use tauri::Wry;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Store = Arc<tauri_plugin_store::Store<Wry>>;

#[derive(Serialize, Debug, PartialEq, Default)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Fetchable<T> {
    /// The value has not been fetched yet.
    #[default]
    NotFetched,
    /// A fetch was attempted, but no value was found. We won't try again.
    NotFound,
    /// The value has been fetched and is available.
    Available(T),
}

/// A private helper enum for deserializing `Fetchable<T>`.
/// This is necessary because items defined inside a function (like a `fn deserialize`)
/// cannot use generic parameters from that function's outer scope.
#[derive(Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
enum FetchableHelper<T> {
    NotFetched,
    NotFound,
    Available(T),
}

impl<'de, T> Deserialize<'de> for Fetchable<T>
where
    T: for<'a> Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de;

        // Deserialize to a generic value to inspect it. This is needed because
        // the source can be either a raw value (like `null` or "string") or
        // a structured object (`{"type": "notFetched"}`).
        let value = serde_json::Value::deserialize(deserializer)?;

        // Handle `null` as NotFound
        if value.is_null() {
            return Ok(Fetchable::NotFound);
        }

        // Try to deserialize as the tagged object format first.
        if let Ok(tagged) = serde_json::from_value::<FetchableHelper<T>>(value.clone()) {
            return Ok(match tagged {
                FetchableHelper::NotFetched => Fetchable::NotFetched,
                FetchableHelper::NotFound => Fetchable::NotFound,
                FetchableHelper::Available(v) => Fetchable::Available(v),
            });
        }

        // If that fails, try to deserialize as the raw value `T`.
        if let Ok(v) = serde_json::from_value::<T>(value) {
            return Ok(Fetchable::Available(v));
        }

        // If all attempts fail, return an error.
        Err(de::Error::custom(
            "data could not be deserialized as a raw value or a Fetchable object",
        ))
    }
}
