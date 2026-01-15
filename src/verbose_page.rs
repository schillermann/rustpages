use crate::{Output, Page, TextPage};
use std::collections::HashMap;

pub struct VerbosePage {
    args: HashMap<String, String>,
}

impl VerbosePage {
    pub fn new() -> Self {
        Self {
            args: HashMap::new(),
        }
    }
}

impl Page for VerbosePage {
    fn with(&self, key: &str, value: &str) -> Box<dyn Page> {
        let mut args = self.args.clone();
        args.insert(key.to_string(), value.to_string());
        Box::new(Self { args })
    }

    fn via(&self, output: Box<dyn Output>) -> Box<dyn Output> {
        let body = self
            .args
            .iter()
            .map(|(key, value)| format!("{key}: {value}"))
            .collect::<Vec<String>>()
            .join("\n");
        TextPage::new(&body).via(output)
    }
}
