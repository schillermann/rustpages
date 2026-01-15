use crate::{Output, Page};

pub struct TextPage {
    body: String,
}

impl TextPage {
    pub fn new(text: &str) -> Self {
        Self {
            body: text.to_string(),
        }
    }
}

impl Page for TextPage {
    fn with(&self, _key: &str, _value: &str) -> Box<dyn Page> {
        Box::new(Self {
            body: self.body.clone(),
        })
    }

    fn via(&self, output: Box<dyn Output>) -> Box<dyn Output> {
        output
            .with("Content-Type", "text/plain")
            .with("Content-Length", &self.body.len().to_string())
            .with("RustPages-Body", &self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::TextPage;
    use crate::{Page, PageWithType, SimpleOutput};

    #[test]
    fn test_works() {
        let page: Box<dyn Page> = Box::new(PageWithType::new(
            Box::new(TextPage::new("Hi!")),
            "text/html",
        ));
        let page = page
            .with("RustPages-Path", "/user/account")
            .with("Accept", "text/html");
        let output = page.via(Box::new(SimpleOutput::new("")));
        let mut bytes = Vec::new();
        output.write_to(&mut bytes).unwrap();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains(""));
    }
}
