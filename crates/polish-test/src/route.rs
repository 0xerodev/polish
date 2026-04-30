#[derive(Debug, Clone)]
pub struct RouteResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl RouteResponse {
    pub fn is_ok(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    pub fn is_redirect(&self) -> bool {
        self.status >= 300 && self.status < 400
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
    }

    pub fn content_type(&self) -> Option<&str> {
        self.header("content-type")
    }

    pub fn assert_status(&self, expected: u16) -> Result<(), String> {
        if self.status == expected {
            Ok(())
        } else {
            Err(format!("Expected status {expected}, got {}", self.status))
        }
    }

    pub fn assert_contains(&self, text: &str) -> Result<(), String> {
        if self.body.contains(text) {
            Ok(())
        } else {
            Err(format!("Response body does not contain: {text:?}"))
        }
    }

    pub fn assert_not_contains(&self, text: &str) -> Result<(), String> {
        if !self.body.contains(text) {
            Ok(())
        } else {
            Err(format!("Response body unexpectedly contains: {text:?}"))
        }
    }
}

pub struct RouteTestClient {
    pub base_url: String,
}

impl RouteTestClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into() }
    }

    pub fn get(&self, path: &str) -> RouteResponse {
        // In real usage, this would make an HTTP request.
        // In test mode, it returns a stub for compilation/structure testing.
        RouteResponse {
            status: 200,
            headers: vec![("content-type".into(), "text/html".into())],
            body: format!("<!-- GET {} -->", path),
        }
    }

    pub fn post(&self, path: &str, body: &str) -> RouteResponse {
        RouteResponse {
            status: 200,
            headers: vec![("content-type".into(), "text/html".into())],
            body: format!("<!-- POST {} body_len={} -->", path, body.len()),
        }
    }
}
