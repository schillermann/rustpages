use crate::{Page, Session, SimpleOutput};
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub struct App {
    page: Box<dyn Page>,
}

impl App {
    pub fn new(page: Box<dyn Page>) -> Self {
        Self { page }
    }

    pub fn start(&self, port: u16) -> io::Result<()> {
        let running = Arc::new(AtomicBool::new(true));
        self.start_with(port, running)
    }

    fn start_with(&self, port: u16, running: Arc<AtomicBool>) -> io::Result<()> {
        let listener = TcpListener::bind(("127.0.0.1", port))?;
        listener.set_nonblocking(true)?;
        while running.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    self.process(stream)?;
                }
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    fn process(&self, mut stream: TcpStream) -> io::Result<()> {
        let mut buffer = [0_u8; 10000];
        let total = stream.read(&mut buffer)?;
        if total == 0 {
            return Ok(());
        }
        let request = String::from_utf8_lossy(&buffer[..total]);
        let page = Session::new(self.page.as_ref()).with(request.as_ref());
        let output = page.via(Box::new(SimpleOutput::new("")));
        output.write_to(&mut stream)?;
        stream.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::App;
    use crate::{Output, Page, TextPage};
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::Duration;

    struct RoutedPage;

    impl Page for RoutedPage {
        fn with(&self, key: &str, value: &str) -> Box<dyn Page> {
            if key != "RustPages-Path" {
                return Box::new(RoutedPage);
            }
            match value {
                "/" => Box::new(TextPage::new("Hello, world!")),
                "/balance" => Box::new(TextPage::new("256")),
                "/id" => Box::new(TextPage::new("yegor")),
                _ => Box::new(TextPage::new("Not found!")),
            }
        }

        fn via(&self, output: Box<dyn Output>) -> Box<dyn Output> {
            output.with("RustPages-Body", "Not found")
        }
    }

    fn fetch(port: u16, path: &str) -> String {
        let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
            path
        );
        stream.write_all(request.as_bytes()).unwrap();
        let mut bytes = Vec::new();
        stream.read_to_end(&mut bytes).unwrap();
        String::from_utf8(bytes).unwrap()
    }

    #[test]
    #[ignore]
    fn test_works() {
        let app = App::new(Box::new(RoutedPage));
        let running = Arc::new(AtomicBool::new(true));
        let client_running = Arc::clone(&running);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            for _ in 0..10 {
                let response = fetch(12345, "/");
                assert!(response.contains("Hello, world!"));
            }
            assert!(fetch(12345, "/balance").contains("256"));
            assert!(fetch(12345, "/id").contains("yegor"));
            client_running.store(false, Ordering::SeqCst);
        });
        app.start_with(12345, running).unwrap();
        handle.join().unwrap();
    }

    #[test]
    fn test_simple() {
        let app = App::new(Box::new(TextPage::new("Hello, world!")));
        let running = Arc::new(AtomicBool::new(true));
        let client_running = Arc::clone(&running);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let response = fetch(23456, "/");
            assert!(response.contains("Hello, world!"));
            client_running.store(false, Ordering::SeqCst);
        });
        app.start_with(23456, running).unwrap();
        handle.join().unwrap();
    }
}
