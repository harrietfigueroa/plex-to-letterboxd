use serde::Deserialize;

// Location of a library section directory (e.g. Movies, TV Shows, etc.)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlexLibrarySectionDirectoryLocation {
    pub id: u32,
}

// Directory for a library section (e.g. Movies, TV Shows, etc.)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlexLibrarySectionsDirectory {
    // Title of the directory (e.g. Movies, TV Shows, etc.)
    pub title: String,

    #[serde(rename(deserialize = "Location"))]
    // Location of the directory (e.g. Movies, TV Shows, etc.)
    pub location: [PlexLibrarySectionDirectoryLocation; 1],
}

// Response from the Plex server's list library sections endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlexLibrarySection {
    // Directories for the library section (e.g. Movies, TV Shows, etc.)
    pub directory: Vec<PlexLibrarySectionsDirectory>,
}
