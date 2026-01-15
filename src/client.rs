use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::library::PlexLibrarySection;
use crate::media_item::PlexMediaItem;
use crate::watch_history::{PlexWatchHistory, PlexWatchHistoryItem};

/// Generic wrapper for Plex API responses
///
/// All Plex API responses are wrapped in a `MediaContainer` object.
/// This generic struct allows you to deserialize different response types
/// while maintaining type safety.
///
/// # Type Parameters
///
/// * `T` - The inner type that represents the actual response data
///
/// # Example
///
/// ```no_run
/// use plex_to_letterboxd::client::MediaContainer;
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct MyResponse {
///     pub size: u32,
///     pub total_size: u32,
/// }
///
/// // JSON: {"MediaContainer": {"size": 10, "total_size": 100}}
/// let container: MediaContainer<MyResponse> = serde_json::from_str(json)?;
/// println!("Size: {}", container.media_container.size);
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaContainer<T> {
    /// The inner MediaContainer data
    ///
    /// This field name matches the JSON key "MediaContainer" from Plex API responses.
    /// The generic type `T` allows this struct to hold different response types.
    #[serde(rename = "MediaContainer")]
    pub media_container: T,
}

impl<T> MediaContainer<T> {
    /// Creates a new MediaContainer with the given inner data
    ///
    /// This is useful for testing or when constructing responses programmatically.
    pub fn new(media_container: T) -> Self {
        Self { media_container }
    }

    /// Extracts the inner MediaContainer data, consuming the wrapper
    ///
    /// This is useful when you want to move the data out of the wrapper.
    pub fn into_inner(self) -> T {
        self.media_container
    }

    /// Gets a reference to the inner MediaContainer data
    pub fn inner(&self) -> &T {
        &self.media_container
    }
}

/// Represents a Plex client that can communicate with a Plex Media Server
///
/// This struct holds the necessary information to communicate with a Plex Media Server:
/// - `base_url`: The base URL of your Plex server (e.g., "http://192.168.1.100:32400")
/// - `token`: Your Plex authentication token
/// - `client`: An HTTP client for making requests
pub struct PlexClient {
    /// Base URL of the Plex Media Server (e.g., "http://192.168.1.100:32400")
    base_url: String,
    /// Plex authentication token
    token: String,
    /// HTTP client for making requests
    client: Client,
}

impl PlexClient {
    /// Creates a new PlexClient with the given server URL and authentication token
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of your Plex Media Server
    ///   Example: "http://192.168.1.100:32400" or "https://your-server.plex.direct:32400"
    /// * `token` - Your Plex authentication token
    ///
    /// # Example
    ///
    /// ```no_run
    /// use plex_to_letterboxd::client::PlexClient;
    ///
    /// let client = PlexClient::new(
    ///     "http://192.168.1.100:32400".to_string(),
    ///     "your-token-here".to_string(),
    /// );
    /// ```
    pub fn new(base_url: String, token: String) -> Self {
        // Create an HTTP client with default settings
        // The `blocking` feature of reqwest gives us a synchronous client
        let client = Client::new();

        Self {
            base_url,
            token,
            client,
        }
    }

    /// Returns an iterator over watch history items with automatic pagination
    ///
    /// This method returns an iterator that automatically handles pagination,
    /// fetching 100 items per request. The iterator yields only `PlexWatchHistoryItem`
    /// values, not the metadata wrapper.
    ///
    /// # Arguments
    ///
    /// * `library_section_id` - The library section ID to filter watch history by
    ///
    /// # Returns
    ///
    /// An iterator that yields `PlexWatchHistoryItem` values. The iterator
    /// automatically fetches additional pages as needed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use plex_to_letterboxd::client::PlexClient;
    ///
    /// let client = PlexClient::new(url, token);
    ///
    /// for item in client.watch_history_iter("1") {
    ///     let item = item?;
    ///     println!("Watched: {} at {}", item.title, item.viewed_at);
    /// }
    /// ```
    pub fn watch_history_iter(&self, library_section_id: &str) -> WatchHistoryIterator<'_> {
        WatchHistoryIterator::new(self, library_section_id)
    }

    pub fn get_media_item_metadata(&self, rating_key: String) -> Result<PlexMediaItem> {
        let container: MediaContainer<PlexMediaItem> = self
            .get_media_container(format!("/library/metadata/{}", rating_key).as_str(), None)
            .context("Failed to get media item metadata")?;
        Ok(container.into_inner())
    }

    pub fn get_library_sections(&self) -> Result<PlexLibrarySection> {
        let container: MediaContainer<PlexLibrarySection> = self
            .get_media_container("/library/sections", None)
            .context("Failed to get library sections")?;
        Ok(container.into_inner())
    }

    /// Gets the base URL of the Plex server
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Makes a generic API request that returns a MediaContainer response
    ///
    /// This is a helper method for making Plex API requests that return
    /// responses wrapped in a MediaContainer. The generic type `T` allows
    /// you to specify the inner response type.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint path (e.g., "/library/sections")
    /// * `query_params` - Optional query parameters as key-value pairs (e.g., `Some(&[("limit", "10"), ("sort", "title")])`)
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the inner MediaContainer data
    ///
    /// Note: The entire response (including the MediaContainer wrapper) is deserialized
    /// as `MediaContainer<T>`. The type `T` represents the inner data structure.
    ///
    /// # Returns
    ///
    /// * `Ok(MediaContainer<T>)` - The parsed response wrapped in MediaContainer
    /// * `Err` - If the request fails or parsing fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use plex_to_letterboxd::client::{PlexClient, MediaContainer};
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, Deserialize)]
    /// #[serde(rename_all = "PascalCase")]
    /// struct LibrarySection {
    ///     pub key: String,
    ///     pub title: String,
    /// }
    ///
    /// let client = PlexClient::new(url, token);
    ///
    /// // Without query parameters
    /// let response: MediaContainer<LibrarySection> =
    ///     client.get_media_container("/library/sections", None)?;
    ///
    /// // With query parameters
    /// let response: MediaContainer<LibrarySection> =
    ///     client.get_media_container("/library/sections", Some(&[("limit", "10"), ("sort", "title")]))?;
    /// ```
    pub fn get_media_container<T>(
        &self,
        endpoint: &str,
        query_params: Option<&[(&str, &str)]>,
    ) -> Result<MediaContainer<T>>
    where
        MediaContainer<T>: for<'de> Deserialize<'de>,
    {
        // Build the full URL
        let url = format!("{}{}", self.base_url, endpoint);

        // Build the request
        let mut request = self
            .client
            .get(&url)
            .header("X-Plex-Token", &self.token)
            .header("Accept", "application/json");

        // Add query parameters if provided
        if let Some(params) = query_params {
            request = request.query(params);
        }

        // Send the request
        let response = request
            .send()
            .context(format!("Failed to send request to endpoint: {}", endpoint))?;

        // Check for HTTP errors
        let response = response.error_for_status().context(format!(
            "Plex server returned an error for endpoint: {}",
            endpoint
        ))?;

        // Parse the entire JSON response as MediaContainer<T>
        // The Plex API returns the entire response wrapped in MediaContainer,
        // so we deserialize the whole response, not just the inner type
        let container: MediaContainer<T> = response.json().context(format!(
            "Failed to parse response from endpoint: {}",
            endpoint
        ))?;

        Ok(container)
    }

    /// Makes a paginated API request for watch history with headers
    ///
    /// This is a specialized method for watch history that uses HTTP headers
    /// for pagination instead of query parameters, as required by the Plex API.
    fn get_watch_history_page(
        &self,
        offset: u32,
        page_size: u32,
        library_section_id: &str,
    ) -> Result<MediaContainer<PlexWatchHistory>> {
        let url = format!("{}/status/sessions/history/all", self.base_url);

        // Convert to strings for headers
        let offset_str = offset.to_string();
        let page_size_str = page_size.to_string();

        // Build the request with pagination headers
        let request = self
            .client
            .get(&url)
            .header("X-Plex-Token", &self.token)
            .header("Accept", "application/json")
            .header("X-Plex-Container-Start", &offset_str)
            .header("X-Plex-Container-Size", &page_size_str)
            .query(&[
                ("sort", "viewedAt:desc"),
                ("librarySectionID", library_section_id),
                ("accountID", "1"),
            ]);

        // Send the request
        let response = request
            .send()
            .context("Failed to send watch history pagination request")?;

        // Check for HTTP errors
        let response = response
            .error_for_status()
            .context("Plex server returned an error for watch history pagination request")?;

        // Parse the response
        let container: MediaContainer<PlexWatchHistory> = response
            .json()
            .context("Failed to parse watch history pagination response")?;

        Ok(container)
    }
}

/// Iterator over watch history items with automatic pagination
///
/// This iterator automatically handles pagination by fetching 100 items per request.
/// It yields only `PlexWatchHistoryItem` values, not the metadata wrapper.
pub struct WatchHistoryIterator<'a> {
    client: &'a PlexClient,
    library_section_id: String,
    current_items: Vec<PlexWatchHistoryItem>,
    current_index: usize,
    offset: u32,
    page_size: u32,
    is_last_page: bool,
}

impl<'a> WatchHistoryIterator<'a> {
    fn new(client: &'a PlexClient, library_section_id: &str) -> Self {
        Self {
            client,
            library_section_id: library_section_id.to_string(),
            current_items: Vec::new(),
            current_index: 0,
            offset: 0,
            page_size: 100,
            is_last_page: false,
        }
    }

    fn fetch_next_page(&mut self) -> Result<bool> {
        // If we've already determined this is the last page, don't fetch again
        if self.is_last_page {
            return Ok(false);
        }

        // Fetch the page using the specialized method with headers
        let container: MediaContainer<PlexWatchHistory> = self
            .client
            .get_watch_history_page(self.offset, self.page_size, &self.library_section_id)
            .context("Failed to fetch watch history page")?;

        let history = container.into_inner();

        // If we got no items, we're done
        if history.metadata.is_empty() {
            return Ok(false);
        }

        // Update current items and reset index
        self.current_items = history.metadata;
        self.current_index = 0;

        // Check if we received fewer items than requested - this means it's the last page
        let items_received = self.current_items.len() as u32;
        if items_received < self.page_size {
            self.is_last_page = true;
        }

        // Update offset for next fetch
        self.offset += items_received;

        // We successfully fetched a page with items
        Ok(true)
    }
}

impl<'a> Iterator for WatchHistoryIterator<'a> {
    type Item = Result<PlexWatchHistoryItem>;

    fn next(&mut self) -> Option<Self::Item> {
        // If we've exhausted the current page, fetch the next one
        if self.current_index >= self.current_items.len() {
            match self.fetch_next_page() {
                Ok(true) => {
                    // Successfully fetched a new page, continue
                }
                Ok(false) => {
                    // No more pages available
                    return None;
                }
                Err(e) => {
                    // Error fetching page
                    return Some(Err(e));
                }
            }
        }

        // If we still have no items after fetching, we're done
        if self.current_items.is_empty() {
            return None;
        }

        // Get the next item and increment index
        let item = self.current_items[self.current_index].clone();
        self.current_index += 1;

        Some(Ok(item))
    }
}
