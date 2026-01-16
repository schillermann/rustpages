use crate::{Output, Page};

pub struct PageWithType {
    origin: Box<dyn Page>,
    kind: String,
}

impl PageWithType {
    pub fn new(page: Box<dyn Page>, content_type: &str) -> Self {
        Self {
            origin: page,
            kind: content_type.to_string(),
        }
    }
}

impl Page for PageWithType {
    fn fresh(&self) -> Box<dyn Page> {
        Box::new(Self::new(self.origin.fresh(), &self.kind))
    }

    fn with(self: Box<Self>, _key: &str, _value: &str) -> Box<dyn Page> {
        self
    }

    fn via(&self, output: Box<dyn Output>) -> Box<dyn Output> {
        self.origin.via(output.with("Content-Type", &self.kind))
    }
}
