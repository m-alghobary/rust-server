#![allow(dead_code)]

pub enum HttpStatusCode {
    Ok,
    NotFound,
    ServerError,
}

impl HttpStatusCode {
    fn get_code(&self) -> usize {
        match &self {
            HttpStatusCode::Ok => 200,
            HttpStatusCode::NotFound => 404,
            HttpStatusCode::ServerError => 500,
        }
    }

    fn get_phrase(&self) -> &str {
        match &self {
            HttpStatusCode::Ok => "OK",
            HttpStatusCode::NotFound => "NOT FOUND",
            HttpStatusCode::ServerError => "INTERNAL SERVER ERROR",
        }
    }
}

pub struct HttpHeader {
    key: String,
    value: String,
}

impl HttpHeader {
    fn parse(&self) -> String {
        format!("{}: {}\r\n", self.key, self.value)
    }
}

pub struct Response {
    pub http_version: String,
    pub status_code: HttpStatusCode,
    pub headers: Vec<HttpHeader>,
    pub body: String,
}

impl Response {
    pub fn ok_from_file(path: &str) -> std::io::Result<Self> {
        Self::from_file(path, HttpStatusCode::Ok)
    }

    pub fn ok(body: &str) -> Self {
        Response {
            http_version: "1.1".to_owned(),
            status_code: HttpStatusCode::Ok,
            headers: vec![],
            body: body.to_owned(),
        }
    }

    pub fn not_found_from_file(path: &str) -> std::io::Result<Self> {
        Self::from_file(path, HttpStatusCode::NotFound)
    }

    pub fn not_found() -> Self {
        Response {
            http_version: "1.1".to_owned(),
            status_code: HttpStatusCode::NotFound,
            headers: vec![],
            body: String::new(),
        }
    }

    fn from_file(path: &str, status_code: HttpStatusCode) -> std::io::Result<Self> {
        let file_content = std::fs::read_to_string(path)?;
        let content_lendth = file_content.len();

        let mut headers: Vec<HttpHeader> = vec![];
        headers.push(HttpHeader {
            key: "Content-Length".to_owned(),
            value: format!("{content_lendth}"),
        });

        headers.push(HttpHeader {
            key: "Content-Type".to_owned(),
            value: String::from("text/html"),
        });

        Ok(Response {
            http_version: "1.1".to_owned(),
            status_code,
            headers,
            body: file_content,
        })
    }
}

impl Response {
    pub fn as_string(&self) -> String {
        let headers: String = self.headers.iter().map(|header| header.parse()).collect();

        format!(
            "HTTP/{} {} {}\r\n{}\r\n{}",
            self.http_version,
            self.status_code.get_code(),
            self.status_code.get_phrase(),
            headers,
            self.body
        )
    }
}
