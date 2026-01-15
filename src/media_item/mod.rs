use serde::Deserialize;

/// Response from the Plex server's list media item metadata endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlexMediaItem {
    pub metadata: [PlexMediaItemMetadata; 1],
}

/// Metadata for a media item
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlexMediaItemMetadata {
    #[serde(rename(deserialize = "Guid"))]
    pub guid: Vec<PlexMediaItemGuidItem>,
}

/// GUID item for a media item (contains identifiers like IMDb ID)
#[derive(Debug, Deserialize)]
pub struct PlexMediaItemGuidItem {
    pub id: String,
}

