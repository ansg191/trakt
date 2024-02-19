# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/ansg191/trakt/compare/trakt-rs-v0.1.2...trakt-rs-v0.2.0) - 2024-02-19

### Other
- Adds genre endpoints
- Adds country endpoints
- Finishes comment endpoints
- Adds POST /comments endpoint
- Adds certifications endpoint
- Removes duplicate code
- Adds tests for auth endpoints
- Adds tests for `iso8601_date`
- Adds tests for checkin endpoints
- Adds `checkin` endpoints
- Adds calendar endpoints
- Replaces `smallstr` with `smol_str`
- Drops `isahc` for `reqwest` in tests
- Adds more show endpoints
- Implements studio endpoint for movies
- Moves serialization & deserialization to smo/
- Adds path param checking to `Request` derive macro
- Adds `EmojiString`
- Adds show endpoints
- Adds `Response` derive macro to `trakt-macros`
