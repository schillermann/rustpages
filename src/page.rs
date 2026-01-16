use crate::Output;

pub trait Page {
    fn fresh(&self) -> Box<dyn Page>;
    fn with(self: Box<Self>, key: &str, value: &str) -> Box<dyn Page>;
    fn via(&self, output: Box<dyn Output>) -> Box<dyn Output>;
}
