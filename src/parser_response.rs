use std::collections::HashMap;

pub struct Response {
    pub body: String,
}

impl Response {
    pub fn new(_builder: ResponseBuilder) -> Self {
        todo!("wait");
    }
}

#[derive(Default)]
pub struct ResponseBuilder {
    version: Option<f32>,
    status: Option<u32>,
    headers: HashMap<String, String>,
    body: String,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version(&mut self, version: f32) {
        self.version = Some(version);
    }

    pub fn status(&mut self, status: u32) {
        self.status = Some(status);
    }

    pub fn header(&mut self, headers: (String, String)) {
        self.headers.insert(headers.0, headers.1);
    }

    pub fn body(&mut self, body: String) {
        self.body = body;
    }
}
