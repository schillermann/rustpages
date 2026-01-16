# ADR 0002: Use Arc for Shared Page Access

Date: 2026-01-16

## Status

Accepted

## Context

The app may become multi-threaded and handle concurrent TCP connections. Sharing a single page instance across threads needs safe, cheap cloning without introducing races.

## Decision

When the app is multi-threaded and expects concurrent TCP connections, use `Arc<dyn Page + Send + Sync>` for shared page access. This keeps sharing safe and cloning cheap per connection.

## Consequences

- Page implementations must satisfy `Send + Sync` bounds when shared.
- Connection handlers can clone `Arc` handles instead of duplicating entire pages.
