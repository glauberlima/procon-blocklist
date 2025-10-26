# Project Architecture Rules (Non-Obvious Only)

- API requests require Mozilla/5.0 user agent string for Procon-SP compatibility
- Add timestamp query parameter "_" to API calls to prevent caching
- Use eprintln! for progress messages, println! only for final output
- Error handling: prefer anyhow::Result over custom error types
- Test data uses hardcoded mock sites instead of API calls
- Always use meaningful branch names
- Always use Conventional Commits pattern when committing changes