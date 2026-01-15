# Plex to Letterboxd

A Rust application that exports your Plex watch history to a CSV file compatible with Letterboxd's import feature. The tool fetches your complete watch history from your Plex Media Server, retrieves IMDb IDs for each item, and generates a CSV file that can be directly uploaded to Letterboxd.

## Getting Started

### Prerequisites

- A Plex Media Server
- Your Plex authentication token
- (Optional) Rust (install from https://rustup.rs/) - only needed if building from source

### Downloading Pre-built Executables

Pre-built executables are available for multiple platforms. Download them from the [latest release](https://github.com/harrietfigueroa/plex-to-letterboxd/releases/latest):

1. Click the link above or navigate to the [Releases](https://github.com/harrietfigueroa/plex-to-letterboxd/releases/latest) page
2. Download the appropriate archive for your platform:
   - `plex-to-letterboxd-linux-x86_64-v{VERSION}.tar.gz` for Linux (x86_64)
   - `plex-to-letterboxd-windows-x86_64-v{VERSION}.zip` for Windows (x86_64)
   - `plex-to-letterboxd-macos-x86_64-v{VERSION}.tar.gz` for macOS (Intel)
   - `plex-to-letterboxd-macos-arm64-v{VERSION}.tar.gz` for macOS (Apple Silicon)
3. Extract the archive to get the executable:
   - On **Windows**: Extract the `.zip` file to get `plex-to-letterboxd.exe`
   - On **Linux/macOS**: Extract the `.tar.gz` file to get the `plex-to-letterboxd` binary

After downloading, make the executable file executable on Linux/macOS:

```bash
chmod +x plex-to-letterboxd
```

Then run it:

```bash
./plex-to-letterboxd --plex-url http://your-server-ip:32400 --plex-token your-plex-token-here
```

On Windows:

```cmd
plex-to-letterboxd.exe --plex-url http://your-server-ip:32400 --plex-token your-plex-token-here
```

### Finding Your Plex Token

You can find your Plex authentication token by:

1. Opening your Plex web interface
2. Opening browser developer tools (F12)
3. Going to the Network tab
4. Making any request to your Plex server
5. Looking for the `X-Plex-Token` header in the request

Alternatively, you can find it in your Plex server's preferences or by checking the URL when logged into Plex Web.

### Running the Application

You can provide configuration via command-line arguments or environment variables. The program requires a Plex server URL and authentication token.

#### Using Command-Line Arguments

```bash
cargo run -- --plex-url http://your-server-ip:32400 --plex-token your-plex-token-here
```

With custom output file:

```bash
cargo run -- --plex-url http://your-server-ip:32400 --plex-token your-plex-token-here --output-csv my_watch_history.csv
```

#### Using Environment Variables

```bash
export PLEX_URL="http://your-server-ip:32400"
export PLEX_TOKEN="your-plex-token-here"
export OUTPUT_CSV="plex_watch_history.csv"  # Optional, defaults to "plex_watch_history.csv"
cargo run
```

On Windows:

```cmd
set PLEX_URL=http://your-server-ip:32400
set PLEX_TOKEN=your-plex-token-here
set OUTPUT_CSV=plex_watch_history.csv
cargo run
```

#### Mixed Usage

You can mix command-line arguments and environment variables. Command-line arguments take precedence:

```bash
export PLEX_TOKEN="your-plex-token-here"
cargo run -- --plex-url http://your-server-ip:32400
```

#### Getting Help

To see all available options:

```bash
cargo run -- --help
```

Or after building:

```bash
cargo build --release
./target/release/plex-to-letterboxd --help
```

#### Error Messages

If required arguments are missing, the program will display helpful error messages and exit. For example:

```
Error: Missing required argument: PLEX_TOKEN
Please provide --plex-token or set the PLEX_TOKEN environment variable.
To find your token, see: https://support.plex.tv/articles/204059436-finding-an-authentication-token-x-plex-token/
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

- ✅ CLI application with command-line argument support
- ✅ Environment variable support for configuration
- ✅ Automatic pagination for large watch histories
- ✅ IMDb ID extraction from Plex metadata
- ✅ CSV export in Letterboxd-compatible format
- ✅ Error handling and progress logging
- ✅ Helpful error messages for missing configuration

## Resources

- [Plex API Documentation](https://developer.plex.tv/pms/#section/API-Info/Pagination)
- [Letterboxd CSV Upload](https://letterboxd.com/import/)
