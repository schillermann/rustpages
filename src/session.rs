use crate::Page;
use std::collections::HashMap;

pub struct Session<'a> {
    page: &'a dyn Page,
}

impl<'a> Session<'a> {
    pub fn new(page: &'a dyn Page) -> Self {
        Self { page }
    }

    pub fn with(&self, request: &str) -> Box<dyn Page> {
        let mut pairs: HashMap<String, String> = HashMap::new();
        let lines: Vec<&str> = request.split("\r\n").collect();
        if lines.is_empty() {
            return self.page.with("", "");
        }
        for line in lines.iter().skip(1) {
            if line.is_empty() {
                continue;
            }
            let mut parts = line.splitn(2, ':');
            let name = parts.next().unwrap_or("").trim();
            let value = parts.next().unwrap_or("").trim();
            if name.is_empty() {
                continue;
            }
            pairs.insert(name.to_string(), value.to_string());
        }
        let mut parts = lines[0].splitn(3, ' ');
        let method = parts.next().unwrap_or("");
        let target = parts.next().unwrap_or("");
        let protocol = parts.next().unwrap_or("");
        pairs.insert("RustPages-Method".to_string(), method.to_string());
        let mut qparts = target.splitn(2, '?');
        let path = qparts.next().unwrap_or("");
        let query = qparts.next().unwrap_or("");
        pairs.insert("RustPages-Path".to_string(), path.to_string());
        pairs.insert("RustPages-Query".to_string(), query.to_string());
        pairs.insert("RustPages-Protocol".to_string(), protocol.to_string());
        let mut target_page: Option<Box<dyn Page>> = None;
        for (key, value) in pairs {
            let next = match target_page {
                None => self.page.with(&key, &value),
                Some(page) => page.with(&key, &value),
            };
            target_page = Some(next);
        }
        target_page.unwrap_or_else(|| self.page.with("", ""))
    }
}

#[cfg(test)]
mod tests {
    use super::Session;
    use crate::{Output, Page, SimpleOutput};

    struct TestPage;

    impl Page for TestPage {
        fn with(&self, _key: &str, _value: &str) -> Box<dyn Page> {
            Box::new(TestPage)
        }

        fn via(&self, output: Box<dyn Output>) -> Box<dyn Output> {
            output
                .with("Content-Type", "text/plain")
                .with("Content-Length", "13")
                .with("RustPages-Body", "Hello, world!")
        }
    }

    #[test]
    fn test_works() {
        let page = TestPage;
        let session = Session::new(&page);
        let page = session.with("GET / HTTP/1.1\r\n");
        let output = page.via(Box::new(SimpleOutput::new("")));
        let mut bytes = Vec::new();
        output.write_to(&mut bytes).unwrap();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains("HTTP/1.1 200 OK\r\n"));
    }
}
