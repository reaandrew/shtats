use std::io::Write;

#[derive(Clone)]
pub struct BufferedOutput (Vec<u8>);

impl BufferedOutput {
    pub fn new() -> Self {
        return BufferedOutput(Vec::new());
    }

    pub fn write(&mut self, data: String) {
        self.0.write_all(data.as_bytes());
    }

    pub fn to_string(self) -> String {
        String::from_utf8(self.0).unwrap()
    }
}
