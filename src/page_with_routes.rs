use crate::{Output, Page};
use std::sync::Arc;

pub struct PageWithRoutes {
    path: String,
    right: Arc<dyn Page>,
    wrong: Arc<dyn Page>,
}

impl PageWithRoutes {
    pub fn new(path: &str, right: Box<dyn Page>, wrong: Box<dyn Page>) -> Self {
        Self {
            path: path.to_string(),
            right: Arc::from(right),
            wrong: Arc::from(wrong),
        }
    }
}

impl Page for PageWithRoutes {
    fn with(&self, key: &str, value: &str) -> Box<dyn Page> {
        if key == "RustPages-Path" {
            if value == self.path {
                return self.right.with(key, value);
            }
            return self.wrong.with(key, value);
        }
        Box::new(Self {
            path: self.path.clone(),
            right: Arc::clone(&self.right),
            wrong: Arc::clone(&self.wrong),
        })
    }

    fn via(&self, output: Box<dyn Output>) -> Box<dyn Output> {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::PageWithRoutes;
    use crate::{Page, SimpleOutput, SimplePage};

    fn render(page: Box<dyn Page>) -> String {
        let output = page.via(Box::new(SimpleOutput::new("")));
        let mut bytes = Vec::new();
        output.write_to(&mut bytes).unwrap();
        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn routes_to_right_page() {
        let page = PageWithRoutes::new(
            "/ok",
            Box::new(SimplePage::new("right")),
            Box::new(SimplePage::new("wrong")),
        );
        let routed = page.with("RustPages-Path", "/ok");
        let text = render(routed);
        assert!(text.contains("right"));
    }

    #[test]
    fn routes_to_wrong_page() {
        let page = PageWithRoutes::new(
            "/ok",
            Box::new(SimplePage::new("right")),
            Box::new(SimplePage::new("wrong")),
        );
        let routed = page.with("RustPages-Path", "/nope");
        let text = render(routed);
        assert!(text.contains("wrong"));
    }
}
