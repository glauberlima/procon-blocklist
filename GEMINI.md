# GEMINI.md: procon-badsites

## Project Overview

This project, `procon-badsites`, contains a Rust-based command-line interface (CLI) tool named `procon-cli`. Its purpose is to fetch a list of untrustworthy websites from the Procon-SP (a Brazilian consumer protection agency) public API and generate blocklists in either "adblock" or "hosts" format.

The core technologies used are:
- **Language:** Rust
- **Key Crates (Libraries):**
  - `clap`: For parsing command-line arguments.
  - `reqwest`: For making asynchronous HTTP requests to the API.
  - `serde`: For deserializing the JSON response from the API.
  - `tokio`: As the asynchronous runtime.
  - `anyhow`: For flexible error handling.
  - `chrono`: For generating timestamps in the list headers.

The application logic is straightforward: it fetches data from a specific URL, parses the JSON to extract a list of site domains, and then formats this list into one of two blocklist styles, adding a descriptive header.

The repository is configured with GitHub Actions for CI/CD:
1.  **Validate PR:** Automatically builds and tests the code on pull requests.
2.  **Create Release:** Automatically builds binaries for Linux, macOS (Intel & Apple Silicon), and Windows, and creates a GitHub Release when new features or fixes are pushed to the `main` branch.
3.  **Generate Lists:** A scheduled workflow runs weekly (Tuesday to Friday) to automatically regenerate the blocklists located in the `lists/` directory and commits the changes.

## Building and Running

The project uses the standard Rust toolchain (Cargo).

### Building

- **For debugging:**
  ```bash
  cargo build
  ```
- **For a release (optimized) build:**
  ```bash
  cargo build --release
  ```
  The binary will be located at `target/release/procon-cli`.

### Running

- **During development:**
  ```bash
  # Generate adblock list to standard output
  cargo run -- generate

  # Generate hosts list and save to a file
  cargo run -- generate --format hosts --output ./hosts.txt
  ```
- **Using the compiled binary:**
  ```bash
  ./target/release/procon-cli generate
  ```

### Testing

- **Run all unit tests:**
  ```bash
  cargo test
  ```

## Development Conventions

- **Code Style:** The code follows standard Rust conventions. While no specific linter is enforced in the CI pipeline, it's recommended to use `rustfmt` for formatting and `clippy` for linting.
  ```bash
  # Format the code
  cargo fmt

  # Run clippy for suggestions
  cargo clippy
  ```
- **Testing:** Unit tests are included in `src/main.rs` within a `#[cfg(test)]` module. New features should be accompanied by corresponding tests.
- **Commits:** The `README.md` suggests following the [Conventional Commits](https://conventionalcommits.org/) specification. The release workflow is triggered by commit messages containing `feat`, `fix`, or `BREAKING`.
- **Branching:** The primary branch is `main`. Feature development should be done on separate branches and merged via pull requests.
