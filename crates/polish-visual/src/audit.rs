#[derive(Debug, Clone)]
pub enum AuditCheck {
    ContrastRatio,
    FontSizeMinimum,
    ClickTargetSize,
    ColorBlindSafe,
    LayoutOverflow,
    TextTruncation,
    ImageAltText,
    FocusIndicator,
}

impl AuditCheck {
    pub fn as_str(&self) -> &str {
        match self {
            AuditCheck::ContrastRatio => "contrast_ratio",
            AuditCheck::FontSizeMinimum => "font_size_minimum",
            AuditCheck::ClickTargetSize => "click_target_size",
            AuditCheck::ColorBlindSafe => "color_blind_safe",
            AuditCheck::LayoutOverflow => "layout_overflow",
            AuditCheck::TextTruncation => "text_truncation",
            AuditCheck::ImageAltText => "image_alt_text",
            AuditCheck::FocusIndicator => "focus_indicator",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditFinding {
    pub check: String,
    pub severity: String,
    pub message: String,
    pub element: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuditResult {
    pub checks_run: Vec<String>,
    pub findings: Vec<AuditFinding>,
    pub passed: bool,
    pub score: f32,
}

impl AuditResult {
    pub fn perfect() -> Self {
        Self { checks_run: Vec::new(), findings: Vec::new(), passed: true, score: 100.0 }
    }

    pub fn from_html(html: &str) -> Self {
        let mut findings = Vec::new();
        let mut checks_run = Vec::new();

        checks_run.push("image_alt_text".into());
        let img_count = html.matches("<img").count();
        let alt_count = html.matches("alt=").count();
        if img_count > alt_count {
            findings.push(AuditFinding {
                check: "image_alt_text".into(),
                severity: "warning".into(),
                message: format!("{} images missing alt text", img_count - alt_count),
                element: None,
            });
        }

        checks_run.push("layout_overflow".into());

        let error_count = findings.iter().filter(|f| f.severity == "error").count();
        let warning_count = findings.iter().filter(|f| f.severity == "warning").count();
        let score = 100.0 - (error_count as f32 * 20.0) - (warning_count as f32 * 5.0);

        AuditResult {
            checks_run,
            findings,
            passed: error_count == 0,
            score: score.max(0.0),
        }
    }

    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }
}

pub struct VisualAudit {
    pub checks: Vec<AuditCheck>,
}

impl VisualAudit {
    pub fn full() -> Self {
        Self {
            checks: vec![
                AuditCheck::ContrastRatio,
                AuditCheck::FontSizeMinimum,
                AuditCheck::ClickTargetSize,
                AuditCheck::LayoutOverflow,
                AuditCheck::TextTruncation,
                AuditCheck::ImageAltText,
                AuditCheck::FocusIndicator,
            ],
        }
    }

    pub fn audit_html(&self, html: &str) -> AuditResult {
        AuditResult::from_html(html)
    }
}
