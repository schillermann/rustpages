# Guidelines

- Avoid stringifying objects to get information; prefer asking them to do the work (e.g., `write_to` over `to_string`).
- Prefer small, composable objects over large, stateful ones.
- Keep public APIs minimal and intention-revealing.
- Favor immutability; return new objects instead of mutating existing state.
- Avoid exposing internal data structures directly.
