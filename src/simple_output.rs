use crate::Output;
use std::io;

pub struct SimpleOutput {
    before: String,
}

impl SimpleOutput {
    pub fn new(text: &str) -> Self {
        Self {
            before: text.to_string(),
        }
    }
}

impl Output for SimpleOutput {
    fn with(self: Box<Self>, name: &str, value: &str) -> Box<dyn Output> {
        let mut after = self.before.clone();
        if after.is_empty() {
            after.push_str("HTTP/1.1 200 OK\r\n");
        }
        if name == "RustPages-Body" {
            after.push_str("\r\n");
            after.push_str(value);
        } else {
            after.push_str(name);
            after.push_str(": ");
            after.push_str(value);
            after.push_str("\r\n");
        }
        Box::new(SimpleOutput { before: after })
    }

    fn write_to(&self, output: &mut dyn io::Write) -> io::Result<()> {
        output.write_all(self.before.as_bytes())
    }
}
