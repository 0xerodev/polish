use std::collections::HashMap;
use crate::field::{Field, FieldMeta};

/// Raw key→value map parsed from an HTTP form POST body.
#[derive(Clone, Debug, Default)]
pub struct ParsedForm(pub HashMap<String, String>);

impl ParsedForm {
    pub fn new() -> Self { Self::default() }

    pub fn from_query(query: &str) -> Self {
        let mut map = HashMap::new();
        for pair in query.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                let key = url_decode(k);
                let val = url_decode(v);
                map.insert(key, val);
            }
        }
        Self(map)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }

    pub fn get_or_empty(&self, key: &str) -> &str {
        self.0.get(key).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn csrf_token(&self) -> Option<&str> {
        self.get("_csrf")
    }
}

/// A typed form definition — schema + runtime state.
#[derive(Clone, Debug, Default)]
pub struct Form {
    pub name: String,
    pub action: String,
    pub method: String,
    pub fields: Vec<FieldMeta>,
    pub values: HashMap<String, String>,
    pub errors: HashMap<String, Vec<String>>,
    pub global_error: Option<String>,
    pub success: Option<String>,
}

impl Form {
    pub fn new(name: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            action: action.into(),
            method: "POST".into(),
            ..Default::default()
        }
    }

    pub fn field(mut self, meta: FieldMeta) -> Self {
        self.fields.push(meta); self
    }

    pub fn with_values(mut self, vals: &ParsedForm) -> Self {
        self.values = vals.0.clone(); self
    }

    pub fn with_errors(mut self, errs: HashMap<String, Vec<String>>) -> Self {
        self.errors = errs; self
    }

    pub fn with_global_error(mut self, e: impl Into<String>) -> Self {
        self.global_error = Some(e.into()); self
    }

    pub fn with_success(mut self, s: impl Into<String>) -> Self {
        self.success = Some(s.into()); self
    }

    pub fn get_value(&self, field: &str) -> &str {
        self.values.get(field).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn get_errors(&self, field: &str) -> &[String] {
        self.errors.get(field).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty() && self.global_error.is_none()
    }

    pub fn runtime_field(&self, name: &str) -> Field {
        Field {
            name: name.to_string(),
            value: self.get_value(name).to_string(),
            errors: self.get_errors(name).to_vec(),
            meta: self.fields.iter().find(|f| f.name == name).cloned(),
            touched: !self.values.is_empty(),
        }
    }
}

/// Runtime form state with full field list for rendering.
#[derive(Clone, Debug, Default)]
pub struct FormState {
    pub form: Form,
    pub is_submitted: bool,
}

impl FormState {
    pub fn new(form: Form) -> Self { Self { form, is_submitted: false } }
    pub fn submitted(mut self) -> Self { self.is_submitted = true; self }
}

fn url_decode(s: &str) -> String {
    let s = s.replace('+', " ");
    let mut out = String::with_capacity(s.len());
    let mut bytes = s.bytes().peekable();
    while let Some(b) = bytes.next() {
        if b == b'%' {
            let mut hex = [0u8; 2];
            hex[0] = bytes.next().unwrap_or(0);
            hex[1] = bytes.next().unwrap_or(0);
            if let Ok(s) = std::str::from_utf8(&hex) {
                if let Ok(n) = u8::from_str_radix(s, 16) {
                    out.push(n as char);
                    continue;
                }
            }
        }
        out.push(b as char);
    }
    out
}

use crate::validator::Validator;
use crate::action::ActionResult;
use crate::error::ActionError;
use crate::csrf::CsrfStore;

/// Full server-action pipeline: CSRF check → validate → user handler → ActionResult.
pub fn execute_pipeline<F>(
    parsed: &ParsedForm,
    csrf_store: &CsrfStore,
    validator: &Validator,
    handler: F,
) -> Result<ActionResult, ActionError>
where
    F: FnOnce(&ParsedForm) -> Result<ActionResult, ActionError>,
{
    let token = parsed.csrf_token().ok_or(ActionError::InvalidCsrf)?;
    if !csrf_store.validate(token) {
        return Err(ActionError::InvalidCsrf);
    }
    let errors = validator.validate(parsed);
    if !errors.is_empty() {
        let first = errors.values().next().and_then(|v| v.first()).cloned().unwrap_or_default();
        return Err(ActionError::InvalidInput(first));
    }
    handler(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_form_query() {
        let f = ParsedForm::from_query("amount=100&dest=0x123&_csrf=abc");
        assert_eq!(f.get("amount"), Some("100"));
        assert_eq!(f.csrf_token(), Some("abc"));
    }

    #[test]
    fn url_decode_plus_space() {
        let f = ParsedForm::from_query("q=hello+world");
        assert_eq!(f.get("q"), Some("hello world"));
    }
}

