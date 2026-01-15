use std::io;

pub trait Output {
    fn with(self: Box<Self>, name: &str, value: &str) -> Box<dyn Output>;
    fn write_to(&self, output: &mut dyn io::Write) -> io::Result<()>;
}
