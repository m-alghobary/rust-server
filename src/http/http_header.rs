#[derive(Debug, Clone)]
pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

impl HttpHeader {
    pub fn parse(&self) -> String {
        format!("{}: {}\r\n", self.key, self.value)
    }
}
