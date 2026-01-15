use crate::{Output, Page};
use std::sync::Arc;

pub struct PageWithType {
    origin: Arc<dyn Page>,
    kind: String,
}

impl PageWithType {
    pub fn new(page: Box<dyn Page>, content_type: &str) -> Self {
        Self {
            origin: Arc::from(page),
            kind: content_type.to_string(),
        }
    }
}

impl Page for PageWithType {
    fn with(&self, _key: &str, _value: &str) -> Box<dyn Page> {
        Box::new(Self {
            origin: Arc::clone(&self.origin),
            kind: self.kind.clone(),
        })
    }

    fn via(&self, output: Box<dyn Output>) -> Box<dyn Output> {
        self.origin.via(output.with("Content-Type", &self.kind))
    }
}
