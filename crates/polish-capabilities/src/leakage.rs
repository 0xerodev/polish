use crate::matrix::CapabilityMatrix;

#[derive(Debug, Clone)]
pub struct LeakageViolation {
    pub component: String,
    pub field: String,
    pub surface: String,
    pub location: String,
    pub reason: String,
}

#[derive(Debug, Default)]
pub struct LeakageReport {
    pub violations: Vec<LeakageViolation>,
    pub checked_html_fields: usize,
    pub checked_api_fields: usize,
}

impl LeakageReport {
    pub fn is_clean(&self) -> bool { self.violations.is_empty() }

    pub fn summary(&self) -> String {
        if self.is_clean() {
            format!("No leakage violations. Checked {} HTML + {} API fields.",
                self.checked_html_fields, self.checked_api_fields)
        } else {
            format!("{} leakage violation(s) found:\n{}",
                self.violations.len(),
                self.violations.iter().map(|v|
                    format!("  [{}] {}.{} in {} — {}", v.surface, v.component, v.field, v.location, v.reason)
                ).collect::<Vec<_>>().join("\n"))
        }
    }
}

/// Scan HTML output for forbidden capability fields.
pub fn scan_html_for_leakage(html: &str, matrix: &CapabilityMatrix) -> LeakageReport {
    let mut report = LeakageReport::default();
    for (component, entry) in matrix.all_forbidden() {
        report.checked_html_fields += 1;
        // Check if the field id or source appears in the HTML
        if html.contains(&entry.id) || html.contains(&entry.source) {
            report.violations.push(LeakageViolation {
                component: component.to_string(),
                field: entry.id.clone(),
                surface: entry.policy.surface.to_string(),
                location: "html_output".to_string(),
                reason: format!("NeverUi/HiddenEnforced field '{}' found in HTML output", entry.id),
            });
        }
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::{CapabilityMatrix, CapabilityRow};
    use crate::surface::{Surface, SurfacePolicy};

    #[test]
    fn detects_leakage() {
        let matrix = CapabilityMatrix::new()
            .add_row(CapabilityRow::new("transfer")
                .add("debug_stack", "internal_error",
                    SurfacePolicy::new(Surface::NeverUi, "debug_stack", "security leakage forbidden")));
        let html = "<div>debug_stack: something</div>";
        let report = scan_html_for_leakage(html, &matrix);
        assert!(!report.is_clean());
    }

    #[test]
    fn clean_html_passes() {
        let matrix = CapabilityMatrix::new()
            .add_row(CapabilityRow::new("transfer")
                .add("debug_stack", "internal_error",
                    SurfacePolicy::new(Surface::NeverUi, "debug_stack", "forbidden")));
        let html = "<div>You receive: 100 USDC</div>";
        let report = scan_html_for_leakage(html, &matrix);
        assert!(report.is_clean());
    }
}
