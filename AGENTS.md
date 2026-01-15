# AGENTS

## Git commit message convention

All commits **must** follow the **Conventional Commits v1.0.0** specification.

### Commit message format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Allowed commit types

- `feat` – a new feature
- `fix` – a bug fix
- `docs` – documentation only changes
- `style` – formatting changes (no code logic changes)
- `refactor` – code changes that neither fix a bug nor add a feature
- `perf` – performance improvements
- `test` – adding or correcting tests
- `build` – changes affecting the build system or external dependencies
- `ci` – changes to CI configuration or scripts
- `chore` – maintenance tasks that do not affect runtime behavior

### Scope (optional but encouraged)

- The scope should be a **noun** describing the affected area:
  - `agents`
  - `rpc`
  - `editor`
  - `infrastructure`
  - `docs`

### Description rules

- Use the **imperative mood**
- Do **not** capitalize the first letter
- Do **not** end with a period
- Keep it concise and specific

### Breaking changes

- Breaking changes **must** be explicitly marked
- Use `!` after the type or a `BREAKING CHANGE:` footer

---

### Enforcement expectations

- Every commit must be parseable by Conventional Commits tooling
- Agents should **reject** commits that do not follow this format
- Squash merges must preserve a valid Conventional Commit message
