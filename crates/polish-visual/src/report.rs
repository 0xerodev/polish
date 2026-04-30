use crate::diff::DiffResult;
use crate::audit::AuditResult;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportStatus {
    Pass,
    Warn,
    Fail,
}

impl ReportStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ReportStatus::Pass => "PASS",
            ReportStatus::Warn => "WARN",
            ReportStatus::Fail => "FAIL",
        }
    }

    pub fn css_class(&self) -> &str {
        match self {
            ReportStatus::Pass => "result-ok",
            ReportStatus::Warn => "result-warn",
            ReportStatus::Fail => "result-error",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReportSection {
    pub title: String,
    pub status: ReportStatus,
    pub description: String,
    pub details: Vec<String>,
    pub screenshot_path: Option<String>,
}

impl ReportSection {
    pub fn pass(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self { title: title.into(), status: ReportStatus::Pass, description: description.into(), details: Vec::new(), screenshot_path: None }
    }

    pub fn warn(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self { title: title.into(), status: ReportStatus::Warn, description: description.into(), details: Vec::new(), screenshot_path: None }
    }

    pub fn fail(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self { title: title.into(), status: ReportStatus::Fail, description: description.into(), details: Vec::new(), screenshot_path: None }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.details.push(detail.into());
        self
    }

    pub fn with_screenshot(mut self, path: impl Into<String>) -> Self {
        self.screenshot_path = Some(path.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct VisualReport {
    pub title: String,
    pub sections: Vec<ReportSection>,
    pub generated_at: String,
    pub overall_status: ReportStatus,
}

impl VisualReport {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            sections: Vec::new(),
            generated_at: chrono_now(),
            overall_status: ReportStatus::Pass,
        }
    }

    pub fn add_section(&mut self, section: ReportSection) {
        if section.status == ReportStatus::Fail {
            self.overall_status = ReportStatus::Fail;
        } else if section.status == ReportStatus::Warn && self.overall_status == ReportStatus::Pass {
            self.overall_status = ReportStatus::Warn;
        }
        self.sections.push(section);
    }

    pub fn from_diff(title: impl Into<String>, diff: &DiffResult) -> Self {
        let mut report = Self::new(title);
        let section = if diff.passed {
            ReportSection::pass("Visual diff", diff.summary())
        } else {
            ReportSection::fail("Visual diff", diff.summary())
                .with_detail(format!("Threshold: {:.2}%, Actual: {:.2}%", diff.threshold_percent, diff.diff_percent))
        };
        report.add_section(section);
        report
    }

    pub fn from_audit(title: impl Into<String>, audit: &AuditResult) -> Self {
        let mut report = Self::new(title);
        let section = if audit.passed {
            ReportSection::pass("Visual audit", format!("Score: {:.0}/100", audit.score))
        } else {
            ReportSection::fail("Visual audit", format!("Score: {:.0}/100, {} findings", audit.score, audit.finding_count()))
        };
        report.add_section(section);
        report
    }

    pub fn to_html(&self) -> String {
        let mut buf = String::new();
        buf.push_str("<!DOCTYPE html><html><head><meta charset=utf-8>");
        buf.push_str(&format!("<title>{}</title>", escape_html(&self.title)));
        buf.push_str("<style>body{font-family:sans-serif;max-width:900px;margin:2rem auto;padding:1rem}");
        buf.push_str(".section{border:1px solid #ddd;border-radius:8px;margin:1rem 0;padding:1rem}");
        buf.push_str(".PASS{border-left:4px solid #22c55e}.WARN{border-left:4px solid #f59e0b}.FAIL{border-left:4px solid #ef4444}");
        buf.push_str(".badge{display:inline-block;padding:2px 8px;border-radius:4px;font-size:12px;font-weight:bold}");
        buf.push_str(".badge-PASS{background:#dcfce7;color:#166534}.badge-WARN{background:#fef9c3;color:#92400e}.badge-FAIL{background:#fee2e2;color:#991b1b}");
        buf.push_str("img{max-width:100%;border:1px solid #ddd;border-radius:4px;margin-top:0.5rem}</style></head><body>");
        buf.push_str(&format!("<h1>{}</h1>", escape_html(&self.title)));
        buf.push_str(&format!("<p>Generated: {} | Overall: <span class=\"badge badge-{}\">{}</span></p>",
            escape_html(&self.generated_at), self.overall_status.as_str(), self.overall_status.as_str()));
        for section in &self.sections {
            buf.push_str(&format!("<div class=\"section {}\">", section.status.as_str()));
            buf.push_str(&format!("<h2>{} <span class=\"badge badge-{}\">{}</span></h2>",
                escape_html(&section.title), section.status.as_str(), section.status.as_str()));
            buf.push_str(&format!("<p>{}</p>", escape_html(&section.description)));
            if !section.details.is_empty() {
                buf.push_str("<ul>");
                for d in &section.details {
                    buf.push_str(&format!("<li>{}</li>", escape_html(d)));
                }
                buf.push_str("</ul>");
            }
            if let Some(path) = &section.screenshot_path {
                buf.push_str(&format!("<img src=\"{}\" alt=\"screenshot\">", escape_html(path)));
            }
            buf.push_str("</div>");
        }
        buf.push_str("</body></html>");
        buf
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    format!("unix:{secs}")
}
