use crate::{Output, Page};

pub struct PageWithRoutes {
    path: String,
    right: Box<dyn Page>,
    wrong: Box<dyn Page>,
}

impl PageWithRoutes {
    pub fn new(path: &str, right: Box<dyn Page>, wrong: Box<dyn Page>) -> Self {
        Self {
            path: path.to_string(),
            right,
            wrong,
        }
    }
}

impl Page for PageWithRoutes {
    fn fresh(&self) -> Box<dyn Page> {
        Box::new(Self::new(
            &self.path,
            self.right.fresh(),
            self.wrong.fresh(),
        ))
    }

    fn with(self: Box<Self>, key: &str, value: &str) -> Box<dyn Page> {
        let PageWithRoutes { path, right, wrong } = *self;
        if key == "RustPages-Path" {
            if value == path {
                return right.with(key, value);
            }
            return wrong.with(key, value);
        }
        Box::new(Self { path, right, wrong })
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
        let page = Box::new(PageWithRoutes::new(
            "/ok",
            Box::new(SimplePage::new("right")),
            Box::new(SimplePage::new("wrong")),
        ));
        let routed = page.with("RustPages-Path", "/ok");
        let text = render(routed);
        assert!(text.contains("right"));
    }

    #[test]
    fn routes_to_wrong_page() {
        let page = Box::new(PageWithRoutes::new(
            "/ok",
            Box::new(SimplePage::new("right")),
            Box::new(SimplePage::new("wrong")),
        ));
        let routed = page.with("RustPages-Path", "/nope");
        let text = render(routed);
        assert!(text.contains("wrong"));
    }
}
