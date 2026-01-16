# RustPages

## Purpose

Rustpages is a framework based on the structure of [jpages framework](https://github.com/yegor256/jpages) and the OOP style guides by [Yegor Bugayenko](https://www.yegor256.com/2014/11/20/seven-virtues-of-good-object.html).
It serves as the foundation for another project to build a terminal-based editor like [NeoVim](https://neovim.io/) with a separate UI and core, communicating via HTTP over TCP.

## Examples

### Web Server

#### Quickstart

```rust
use rustpages::{Server, TextPage};

fn main() -> std::io::Result<()> {
    let app = Server::new(Box::new(TextPage::new("Hello, world!")));
    app.start(8080)
}
```

#### Routing

```rust
use rustpages::{Page, PageWithRoutes, Server, SimplePage, TextPage};

fn main() -> std::io::Result<()> {
    let app = Server::new(Box::new(PageWithRoutes::new(
        "/",
        Box::new(TextPage::new("Hello, world!")),
        Box::new(SimplePage::new("Not found!")),
    )));
    app.start(8080)
}
```

### TUI

#### Quickstart

This keeps the core and UI as separate processes. The core runs a TCP server
exposing `/state` and `/cmd` over HTTP; the UI connects as a TCP client, sends
commands, and renders state in a loop.

##### Core

```rust
use rustpages::{Output, Page, Server, SimpleOutput, TextPage};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct EditorPage {
    state: Arc<Mutex<String>>,
    path: String,
    query: String,
}

impl EditorPage {
    fn new(state: Arc<Mutex<String>>) -> Self {
        Self {
            state,
            path: String::new(),
            query: String::new(),
        }
    }

    fn with_path(&self, path: &str) -> Self {
        let mut next = self.clone();
        next.path = path.to_string();
        next
    }

    fn with_query(&self, query: &str) -> Self {
        let mut next = self.clone();
        next.query = query.to_string();
        next
    }
}

impl Page for EditorPage {
    fn with(&self, key: &str, value: &str) -> Box<dyn Page> {
        match key {
            "RustPages-Path" => Box::new(self.with_path(value)),
            "RustPages-Query" => Box::new(self.with_query(value)),
            _ => Box::new(self.clone()),
        }
    }

    fn via(&self, output: Box<dyn Output>) -> Box<dyn Output> {
        match self.path.as_str() {
            "/state" => {
                let buf = self.state.lock().unwrap().clone();
                output.with("RustPages-Body", &buf)
            }
            "/cmd" => {
                if let Some((_, text)) = self.query.split_once("insert=") {
                    self.state.lock().unwrap().push_str(text);
                }
                output.with("RustPages-Body", "ok")
            }
            _ => TextPage::new("not found").via(output),
        }
    }
}

fn main() -> std::io::Result<()> {
    let state = Arc::new(Mutex::new(String::new()));
    Server::new(Box::new(EditorPage::new(state))).start(8080)
}
```

##### UI

```rust
use rustpages::{Terminal, TextPage};

fn main() -> std::io::Result<()> {
    let terminal = Terminal::new(Box::new(TextPage::new("Hello, world!")));
    terminal.start(8080)
}
```

## Development

- [Guidelines for local design rules](GUIDELINES.md)
- [Decisions (ADRs)](docs/decisions/README.md)
