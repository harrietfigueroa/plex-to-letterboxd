use anyhow::{Context, Result};
use clap::Parser;
use csv::Writer;
use plex_to_letterboxd::client::PlexClient;

/// Export your Plex watch history to a CSV file compatible with Letterboxd's import feature.
#[derive(Parser, Debug)]
#[command(name = "plex-to-letterboxd")]
#[command(about = "Export Plex watch history to Letterboxd-compatible CSV", long_about = None)]
struct Args {
    /// Plex Media Server URL (e.g., http://192.168.1.100:32400)
    /// Can also be set via PLEX_URL environment variable
    #[arg(long, env = "PLEX_URL")]
    plex_url: Option<String>,

    /// Plex authentication token
    /// Can also be set via PLEX_TOKEN environment variable
    #[arg(long, env = "PLEX_TOKEN")]
    plex_token: Option<String>,

    /// Library name to filter watch history (e.g., "Movies")
    #[arg(long, required = true)]
    library_name: String,

    /// Output CSV file path (defaults to "plex_watch_history.csv")
    /// Can also be set via OUTPUT_CSV environment variable
    #[arg(long, default_value = "plex_watch_history.csv", env = "OUTPUT_CSV")]
    output_csv: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate required environment variables/arguments
    let base_url = args.plex_url.context(
        "Missing required argument: PLEX_URL\n\
         Please provide --plex-url or set the PLEX_URL environment variable.\n\
         Example: --plex-url http://192.168.1.100:32400",
    )?;

    let token = args.plex_token.context(
        "Missing required argument: PLEX_TOKEN\n\
         Please provide --plex-token or set the PLEX_TOKEN environment variable.\n\
         To find your token, see: https://support.plex.tv/articles/204059436-finding-an-authentication-token-x-plex-token/",
    )?;

    if token.is_empty() {
        anyhow::bail!(
            "PLEX_TOKEN cannot be empty\n\
             Please provide a valid token via --plex-token or set the PLEX_TOKEN environment variable.\n\
             To find your token, see: https://support.plex.tv/articles/204059436-finding-an-authentication-token-x-plex-token/"
        );
    }

    // Create a new Plex client
    let client = PlexClient::new(base_url, token);

    // Get library sections to find the matching library
    let library_sections = client
        .get_library_sections()
        .context("Failed to get library sections")?;

    // Find the directory matching the library name
    let library_directory = library_sections
        .directory
        .iter()
        .find(|dir| dir.title == args.library_name)
        .with_context(|| {
            format!(
                "Library '{}' not found. Available libraries: {}",
                args.library_name,
                library_sections
                    .directory
                    .iter()
                    .map(|dir| dir.title.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    // Extract the location ID from the directory's first location
    let location_id = library_directory
        .location
        .first()
        .map(|loc| loc.id.to_string())
        .context("Library directory has no location ID")?;

    // Create CSV writer
    let output_file = &args.output_csv;
    let mut wtr = Writer::from_path(output_file)
        .with_context(|| format!("Failed to create output file: {}", output_file))?;

    // Write CSV header
    wtr.write_record(&["Title", "imdbID", "WatchedDate", "Tags"])?;
    let tags = "\"Imported from Plex\"".to_string();

    // Loop over watch history items using paginated iterator
    // The iterator automatically handles pagination (100 items per request)
    // Pass the location ID to filter by library section
    for item_result in client.watch_history_iter(&location_id.to_string()) {
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

    println!("\n✓ CSV file successfully generated: {}", output_file);
    println!("Upload your watch history at: https://letterboxd.com/import/");

    Ok(())
}
