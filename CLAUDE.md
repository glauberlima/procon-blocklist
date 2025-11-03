# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust CLI tool that generates blocklists (AdBlock and hosts formats) from Procon-SP's (São Paulo Consumer Protection Foundation) database of untrusted websites. The tool fetches data from the public API and generates formatted blocklists that can be used in ad blockers or system hosts files.

## Development Commands

### Build and Run
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run in development
cargo run -- generate

# Run release binary
./target/release/procon-cli generate

# Generate specific formats
./target/release/procon-cli generate --format adblock --output adblock.txt
./target/release/procon-cli generate --format hosts --output hosts
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Linting and Formatting
```bash
# Check code formatting
cargo fmt -- --check

# Format code
cargo fmt

# Run clippy for lints
cargo clippy
```

## Architecture

### Single-File Structure
The entire application is contained in `src/main.rs` with a simple, straightforward architecture:

1. **CLI Layer** (lines 1-25): Uses `clap` for command-line argument parsing
   - Single `generate` subcommand with `--format` and `--output` options

2. **Data Fetching** (`fetch_sites()`, lines 63-83):
   - Makes HTTP request to Procon-SP API
   - Endpoint: `https://sistemas.procon.sp.gov.br/evitesite/list/evitesite.php`
   - Fetches up to 600 sites (jtPageSize=600)
   - Parses JSON response and extracts site URLs from `Records[].strSite`

3. **Format Generators**:
   - `generate_adblock()` (lines 85-105): Creates AdBlock filter list format with `||domain^` rules
   - `generate_hosts()` (lines 107-126): Creates hosts file format with `0.0.0.0 domain` entries
   - Both include metadata headers with title, description, license, and timestamp

4. **Output Handler** (`generate_list()`, lines 40-61):
   - Writes to file if `--output` specified, otherwise prints to stdout
   - Progress messages go to stderr, data to stdout

### Key Dependencies
- **clap**: CLI argument parsing (derive API)
- **reqwest**: HTTP client with native-tls
- **serde/serde_json**: JSON deserialization
- **tokio**: Async runtime (rt-multi-thread)
- **chrono**: Timestamp formatting for headers
- **anyhow**: Error handling

### Release Profile
The `Cargo.toml` includes aggressive optimization settings for small, fast binaries:
- LTO enabled
- Single codegen unit
- Panic=abort
- Symbols stripped

## CI/CD Workflows

### PR Validation (`.github/workflows/validate-pr.yml`)
Runs on PRs to main when Rust code or Cargo files change:
1. Runs `cargo test`
2. Runs `cargo build --release`

### Automated List Generation (`.github/workflows/generate-lists.yml`)
Runs Tuesday-Friday at 02:30 UTC via cron schedule:
1. Downloads latest pre-compiled Linux binary from releases
2. Generates both AdBlock and hosts formats
3. Compares site listings (ignoring timestamp changes)
4. Only commits if actual site list changed

## Testing Strategy

Tests in `src/main.rs` (lines 128-165):
- `test_mock_sites_structure`: Validates data structure handling
- `test_generate_adblock`: Verifies AdBlock format generation
- `test_generate_hosts`: Verifies hosts format generation

All tests use mock data, no network calls to actual API.

## Important Notes

- The project uses Rust edition 2024 (requires Rust 1.91+)
- API responses may change format - the code expects JSON with `Records` array
- The tool adds timestamps to headers, so file changes don't always mean content changes
- Generated files are committed to `data/` directory for GitHub-hosted consumption
- Always use Conventional Commits pattern when committing