# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Commands

- Run single test: `cargo test <test_name>` (e.g., `cargo test test_generate_adblock`)
- Build optimized binary: `cargo build --release`

## Code Style

- API requests require Mozilla/5.0 user agent string for Procon-SP compatibility
- Add timestamp query parameter "_" to API calls to prevent caching
- Use eprintln! for progress messages, println! only for final output
- Error handling: prefer anyhow::Result over custom error types
- Test data uses hardcoded mock sites instead of API calls
- Always use meaningful branch names
- Always use Conventional Commits pattern when committing changes