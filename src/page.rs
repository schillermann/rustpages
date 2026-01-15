use crate::Output;

pub trait Page {
    fn with(&self, key: &str, value: &str) -> Box<dyn Page>;
    fn via(&self, output: Box<dyn Output>) -> Box<dyn Output>;
}
