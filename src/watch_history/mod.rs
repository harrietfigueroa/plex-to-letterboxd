use serde::Deserialize;

use crate::deserializers;

/// Response from the Plex server's list watch history endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlexWatchHistory {
    pub metadata: Vec<PlexWatchHistoryItem>,
    /// Number of items in this response
    #[serde(default)]
    pub size: u32,
    /// Total number of items available
    #[serde(default)]
    pub total_size: u32,
}

/// Individual item in the watch history
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlexWatchHistoryItem {
    /// The title of the media item
    pub title: String,
    pub rating_key: Option<String>,
    #[serde(rename(deserialize = "librarySectionID"))]
    pub library_section_id: String,
    /// The date and time when the item was viewed, formatted as a string
    #[serde(deserialize_with = "deserializers::deserialize_viewed_at")]
    pub viewed_at: String,
}
