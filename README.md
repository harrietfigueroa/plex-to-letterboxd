# Plex to Letterboxd

A Rust application that exports your Plex watch history to a CSV file compatible with Letterboxd's import feature. The tool fetches your complete watch history from your Plex Media Server, retrieves IMDb IDs for each item, and generates a CSV file that can be directly uploaded to Letterboxd.

## Getting Started

### Prerequisites

- Rust (install from https://rustup.rs/)
- A Plex Media Server
- Your Plex authentication token

### Finding Your Plex Token

You can find your Plex authentication token by:

1. Opening your Plex web interface
2. Opening browser developer tools (F12)
3. Going to the Network tab
4. Making any request to your Plex server
5. Looking for the `X-Plex-Token` header in the request

Alternatively, you can find it in your Plex server's preferences or by checking the URL when logged into Plex Web.

### Running the Application

Set environment variables and run:

```bash
export PLEX_URL="http://your-server-ip:32400"
export PLEX_TOKEN="your-plex-token-here"
export OUTPUT_CSV="plex_watch_history.csv"  # Optional, defaults to "plex_watch_history.csv"
cargo run
```

Or on Windows:

```cmd
set PLEX_URL=http://your-server-ip:32400
set PLEX_TOKEN=your-plex-token-here
set OUTPUT_CSV=plex_watch_history.csv
cargo run
```

The application will:

1. Connect to your Plex server
2. Fetch all watch history items (with progress output)
3. Retrieve IMDb IDs for each item
4. Generate a CSV file ready for Letterboxd import

Once complete, upload the generated CSV file to [Letterboxd's import page](https://letterboxd.com/import/).

## How It Works

1. **Connects to your Plex Media Server** using your server URL and authentication token
2. **Fetches watch history** with automatic pagination (100 items per request)
3. **Retrieves metadata** for each watched item to extract IMDb IDs
4. **Generates a CSV file** in Letterboxd's import format with columns:
   - `Title` - The title of the movie/show
   - `imdbID` - The IMDb identifier (e.g., `tt1234567`)
   - `WatchedDate` - The date and time when you watched it
   - `Tags` - Tags for the entry (defaults to "Imported from Plex")

## Project Structure

- `src/main.rs` - Entry point that orchestrates the export process
- `src/lib.rs` - Library root, exports modules
- `src/client.rs` - Plex API client with pagination support
- `src/watch_history/` - Watch history data structures
- `src/media_item/` - Media item metadata structures
- `src/deserializers.rs` - Custom deserializers for Plex API responses

## Features

- ✅ Automatic pagination for large watch histories
- ✅ IMDb ID extraction from Plex metadata
- ✅ CSV export in Letterboxd-compatible format
- ✅ Error handling and progress logging

## Resources

- [Plex API Documentation](https://developer.plex.tv/pms/#section/API-Info/Pagination)
- [Letterboxd CSV Upload](https://letterboxd.com/import/)
