
pub struct BufferedOutput {
    data: String,
}

impl BufferedOutput {
    pub fn new() -> Self {
        Self { data: "".into() }
    }

    pub fn write(&mut self, data: String) {
        self.data = data;
    }

    pub fn to_string(&self) -> String {
        return self.data.clone();
    }
}
