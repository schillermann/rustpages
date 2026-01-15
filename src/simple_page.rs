use crate::{Output, Page};

pub struct SimplePage {
    body: String,
}

impl SimplePage {
    pub fn new(text: &str) -> Self {
        Self {
            body: text.to_string(),
        }
    }
}

impl Page for SimplePage {
    fn with(&self, _key: &str, _value: &str) -> Box<dyn Page> {
        Box::new(Self {
            body: self.body.clone(),
        })
    }

    fn via(&self, output: Box<dyn Output>) -> Box<dyn Output> {
        output
            .with("Content-Length", &self.body.len().to_string())
            .with("RustPages-Body", &self.body)
    }
}
