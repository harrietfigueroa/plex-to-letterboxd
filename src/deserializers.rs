use serde::{Deserialize, Deserializer};

/// Custom deserializer that converts a Unix timestamp (u64) to a formatted date string
///
/// This function is used to deserialize Plex API timestamps (Unix epoch in seconds)
/// into a formatted date string (YYYY-MM-DD format).
///
/// # Arguments
///
/// * `deserializer` - The Serde deserializer
///
/// # Returns
///
/// * `Ok(String)` - A formatted date string
/// * `Err` - If the timestamp is invalid or deserialization fails
///
/// # Example
///
/// ```rust
/// use serde::Deserialize;
/// use plex_to_letterboxd::deserializers::deserialize_viewed_at;
///
/// #[derive(Deserialize)]
/// struct MyStruct {
///     #[serde(deserialize_with = "deserialize_viewed_at")]
///     pub viewed_at: String,
/// }
/// ```
pub fn deserialize_viewed_at<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp: u64 = Deserialize::deserialize(deserializer)?;
    // Format as ISO 8601 date string (e.g., "2024-01-15")
    // Plex timestamps are in seconds since Unix epoch
    let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .ok_or_else(|| serde::de::Error::custom("Invalid timestamp"))?;
    Ok(datetime.format("%Y-%m-%d").to_string())
}

