# ADR 0001: Prototype-Based Page Creation

Date: 2026-01-16

## Status

Accepted

## Context

The server needs a new page instance per request while keeping pages immutable and avoiding shared mutable state across threads.

## Decision

Use `Page::fresh()` to create a new page instance per request. Implement `fresh()` via `Self::new(...)` so object creation flows through constructors.
Keep `Page::with(self: Box<Self>, ...)` consuming and returning a new page to preserve immutability and composability.

## Consequences

- The server holds a prototype `Page` and calls `fresh()` for each request.
- `Page` implementations provide constructors that capture any required initial configuration.
