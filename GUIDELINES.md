# Guidelines

- Prefer small, composable objects over large, stateful ones.
- Keep public APIs minimal and intention-revealing.
- Favor immutability; return new objects instead of mutating existing state.
- Avoid exposing internal data structures directly.

## Rust-Specific

- Avoid stringifying objects to get information; prefer asking them to do the work (e.g., `write_to` over `to_string`).
- Avoid `Into<String>` in public APIs; accept `&str` and allocate explicitly when needed.
