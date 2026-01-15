use anyhow::Result;
use csv::Writer;
use plex_to_letterboxd::client::PlexClient;

fn main() -> Result<()> {
    // Example usage of the PlexClient
    // Replace these with your actual Plex server URL and token

    // Get the server URL from environment variable or use a default
    let base_url =
        std::env::var("PLEX_URL").unwrap_or_else(|_| "http://10.0.0.105:32400".to_string());

    // Get the token from environment variable
    // You can get your token from: https://support.plex.tv/articles/204059436-finding-an-authentication-token-x-plex-token/
    let token = std::env::var("PLEX_TOKEN").unwrap_or_else(|_| "".to_string());

    // Create a new Plex client
    let client = PlexClient::new(base_url, token);

    // Create CSV writer
    let output_file =
        std::env::var("OUTPUT_CSV").unwrap_or_else(|_| "plex_watch_history.csv".to_string());
    let mut wtr = Writer::from_path(&output_file)?;

    // Write CSV header
    wtr.write_record(&["Title", "imdbID", "WatchedDate", "Tags"])?;
    let tags = "\"Imported from Plex\"".to_string();

    // Loop over watch history items using paginated iterator
    // The iterator automatically handles pagination (100 items per request)
    for item_result in client.watch_history_iter() {
        let item = item_result?;
        println!("Processing: {}", item.title);

        // Use pattern matching to safely extract rating_key
        let Some(rating_key) = &item.rating_key else {
            println!("  Skipping {}: missing rating_key or key", item.title);
            continue;
        };

        let media_item_metadata = client.get_media_item_metadata(rating_key.clone())?;
        let guid = media_item_metadata.metadata[0]
            .guid
            .first()
            .map(|g| g.id.as_str().trim_start_matches("imdb://"));

        // Use pattern matching to safely extract guid
        let Some(guid) = guid else {
            println!("  Skipping {}: missing guid", item.title);
            continue;
        };

        // Write row to CSV
        wtr.write_record(&[&item.title, guid, &item.viewed_at, &tags])?;
    }

    // Flush the writer to ensure all data is written
    wtr.flush()?;

    Ok(())
}
