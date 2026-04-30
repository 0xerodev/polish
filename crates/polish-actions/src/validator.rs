use std::collections::HashMap;
use crate::form::ParsedForm;

pub type ValidationResult = Result<(), Vec<ValidationError>>;

#[derive(Clone, Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self { field: field.into(), message: message.into() }
    }
}

/// Composable validators for form fields.
pub struct Validator {
    rules: Vec<Box<dyn ValidatorRule>>,
    labels: HashMap<String, String>,
}

impl Validator {
    pub fn new() -> Self { Self { rules: Vec::new(), labels: HashMap::new() } }

    /// Map an internal field key to a human-readable label used in error messages.
    pub fn label(mut self, field: &str, display: &str) -> Self {
        self.labels.insert(field.to_string(), display.to_string());
        self
    }

    pub fn required(mut self, field: &str) -> Self {
        self.rules.push(Box::new(Required { field: field.to_string() }));
        self
    }

    pub fn min_len(mut self, field: &str, min: usize) -> Self {
        self.rules.push(Box::new(MinLen { field: field.to_string(), min }));
        self
    }

    pub fn max_len(mut self, field: &str, max: usize) -> Self {
        self.rules.push(Box::new(MaxLen { field: field.to_string(), max }));
        self
    }

    pub fn numeric(mut self, field: &str) -> Self {
        self.rules.push(Box::new(Numeric { field: field.to_string() }));
        self
    }

    pub fn positive(mut self, field: &str) -> Self {
        self.rules.push(Box::new(Positive { field: field.to_string() }));
        self
    }

    pub fn min_value(mut self, field: &str, min: f64) -> Self {
        self.rules.push(Box::new(MinValue { field: field.to_string(), min }));
        self
    }

    pub fn max_value(mut self, field: &str, max: f64) -> Self {
        self.rules.push(Box::new(MaxValue { field: field.to_string(), max }));
        self
    }

    pub fn email(mut self, field: &str) -> Self {
        self.rules.push(Box::new(EmailRule { field: field.to_string() }));
        self
    }

    pub fn evm_address(mut self, field: &str) -> Self {
        self.rules.push(Box::new(EvmAddress { field: field.to_string() }));
        self
    }

    pub fn validate(&self, form: &ParsedForm) -> HashMap<String, Vec<String>> {
        let mut errors: HashMap<String, Vec<String>> = HashMap::new();
        for rule in &self.rules {
            if let Err(e) = rule.check(form) {
                errors.entry(e.field.clone()).or_default().push(e.message.clone());
            }
        }
        // Apply display labels: replace first occurrence of field key in each message
        if self.labels.is_empty() { return errors; }
        errors.into_iter().map(|(field, msgs)| {
            let msgs = if let Some(label) = self.labels.get(&field) {
                msgs.into_iter().map(|m| m.replacen(&field, label, 1)).collect()
            } else { msgs };
            (field, msgs)
        }).collect()
    }

    pub fn is_valid(&self, form: &ParsedForm) -> bool {
        self.validate(form).is_empty()
    }
}

impl Default for Validator {
    fn default() -> Self { Self::new() }
}

trait ValidatorRule: Send + Sync {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError>;
}

struct Required { field: String }
impl ValidatorRule for Required {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError> {
        if form.get(&self.field).map(|v| v.trim().is_empty()).unwrap_or(true) {
            Err(ValidationError::new(&self.field, format!("{} is required", &self.field)))
        } else { Ok(()) }
    }
}

struct MinLen { field: String, min: usize }
impl ValidatorRule for MinLen {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError> {
        let v = form.get(&self.field).unwrap_or("");
        if v.len() < self.min {
            Err(ValidationError::new(&self.field, format!("{} must be at least {} characters", &self.field, self.min)))
        } else { Ok(()) }
    }
}

struct MaxLen { field: String, max: usize }
impl ValidatorRule for MaxLen {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError> {
        let v = form.get(&self.field).unwrap_or("");
        if v.len() > self.max {
            Err(ValidationError::new(&self.field, format!("{} must be at most {} characters", &self.field, self.max)))
        } else { Ok(()) }
    }
}

struct Numeric { field: String }
impl ValidatorRule for Numeric {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError> {
        let v = form.get(&self.field).unwrap_or("");
        if v.trim().is_empty() { return Ok(()); }
        v.parse::<f64>().map(|_| ()).map_err(|_|
            ValidationError::new(&self.field, format!("{} must be a number", &self.field))
        )
    }
}

struct Positive { field: String }
impl ValidatorRule for Positive {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError> {
        let v = form.get(&self.field).unwrap_or("0");
        if v.trim().is_empty() { return Ok(()); }
        match v.parse::<f64>() {
            Ok(n) if n > 0.0 => Ok(()),
            _ => Err(ValidationError::new(&self.field, format!("{} must be positive", &self.field)))
        }
    }
}

struct MinValue { field: String, min: f64 }
impl ValidatorRule for MinValue {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError> {
        let v = form.get(&self.field).unwrap_or("0");
        if v.trim().is_empty() { return Ok(()); }
        match v.parse::<f64>() {
            Ok(n) if n >= self.min => Ok(()),
            _ => Err(ValidationError::new(&self.field, format!("{} must be at least {}", &self.field, self.min)))
        }
    }
}

struct MaxValue { field: String, max: f64 }
impl ValidatorRule for MaxValue {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError> {
        let v = form.get(&self.field).unwrap_or("0");
        if v.trim().is_empty() { return Ok(()); }
        match v.parse::<f64>() {
            Ok(n) if n <= self.max => Ok(()),
            _ => Err(ValidationError::new(&self.field, format!("{} must be at most {}", &self.field, self.max)))
        }
    }
}

struct EmailRule { field: String }
impl ValidatorRule for EmailRule {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError> {
        let v = form.get(&self.field).unwrap_or("");
        if v.trim().is_empty() { return Ok(()); }
        if v.contains('@') && v.contains('.') { Ok(()) }
        else { Err(ValidationError::new(&self.field, "Enter a valid email address")) }
    }
}

struct EvmAddress { field: String }
impl ValidatorRule for EvmAddress {
    fn check(&self, form: &ParsedForm) -> Result<(), ValidationError> {
        let v = form.get(&self.field).unwrap_or("").trim();
        if v.is_empty() { return Ok(()); }
        let hex = v.strip_prefix("0x").unwrap_or(v);
        if hex.len() == 40 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(())
        } else {
            Err(ValidationError::new(&self.field, "Enter a valid Ethereum address (0x...)"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::ParsedForm;

    fn form(pairs: &[(&str, &str)]) -> ParsedForm {
        let mut m = std::collections::HashMap::new();
        for (k, v) in pairs { m.insert(k.to_string(), v.to_string()); }
        ParsedForm(m)
    }

    #[test]
    fn required_catches_empty() {
        let v = Validator::new().required("amount");
        let errs = v.validate(&form(&[("amount", "")]));
        assert!(errs.contains_key("amount"));
    }

    #[test]
    fn valid_evm_address() {
        let v = Validator::new().evm_address("dest");
        let errs = v.validate(&form(&[("dest", "0x29E90c48082B5cC84Cfc47a3aDF87e3F3f1d86E1")]));
        assert!(errs.is_empty());
    }

    #[test]
    fn invalid_evm_address() {
        let v = Validator::new().evm_address("dest");
        let errs = v.validate(&form(&[("dest", "not-an-address")]));
        assert!(errs.contains_key("dest"));
    }
}
