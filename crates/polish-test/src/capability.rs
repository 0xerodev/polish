#[derive(Debug, Clone)]
pub struct LeakageTestResult {
    pub passed: bool,
    pub violations: Vec<String>,
    pub checked_fields: usize,
}

impl LeakageTestResult {
    pub fn assert_clean(&self) -> Result<(), String> {
        if self.passed {
            Ok(())
        } else {
            Err(format!("Leakage violations found: {:?}", self.violations))
        }
    }
}

pub struct CapabilityTestHarness {
    forbidden_patterns: Vec<String>,
}

impl CapabilityTestHarness {
    pub fn new() -> Self {
        Self { forbidden_patterns: Vec::new() }
    }

    pub fn forbid(mut self, pattern: impl Into<String>) -> Self {
        self.forbidden_patterns.push(pattern.into());
        self
    }

    pub fn check_html(&self, html: &str) -> LeakageTestResult {
        let mut violations = Vec::new();
        for pattern in &self.forbidden_patterns {
            if html.contains(pattern.as_str()) {
                violations.push(format!("Forbidden pattern found in HTML: {pattern:?}"));
            }
        }
        LeakageTestResult {
            passed: violations.is_empty(),
            violations,
            checked_fields: self.forbidden_patterns.len(),
        }
    }
}

impl Default for CapabilityTestHarness {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_secret_in_html() {
        let harness = CapabilityTestHarness::new().forbid("SECRET_KEY");
        let html = "<p>Your key is: SECRET_KEY_abc123</p>";
        let result = harness.check_html(html);
        assert!(!result.passed);
        assert_eq!(result.violations.len(), 1);
    }

    #[test]
    fn clean_html_passes() {
        let harness = CapabilityTestHarness::new().forbid("SECRET_KEY");
        let html = "<p>Hello world</p>";
        let result = harness.check_html(html);
        assert!(result.passed);
    }
}
