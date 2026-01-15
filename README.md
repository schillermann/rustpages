# RustPages

## Purpose

Rustpages is a framework based on the structure of [jpages framework](https://github.com/yegor256/jpages) and the OOP style guides by [Yegor Bugayenko](https://www.yegor256.com/2014/11/20/seven-virtues-of-good-object.html).
It serves as the foundation for another project to build a terminal-based editor like [NeoVim](https://neovim.io/) with a separate UI and core, communicating via HTTP over TCP.

## Quickstart

```rust
use rustpages::{App, TextPage};

fn main() -> std::io::Result<()> {
    let app = App::new(Box::new(TextPage::new("Hello, world!")));
    app.start(8080)
}
```

## Examples

### Routing

```rust
use rustpages::{App, Page, PageWithRoutes, SimplePage, TextPage};

fn main() -> std::io::Result<()> {
    let app = App::new(Box::new(PageWithRoutes::new(
        "/",
        Box::new(TextPage::new("Hello, world!")),
        Box::new(SimplePage::new("Not found!")),
    )));
    app.start(8080)
}
```

## Development

- [Guidelines for local design rules](GUIDELINES.md)
