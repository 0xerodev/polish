#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReviewSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl ReviewSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            ReviewSeverity::Info => "info",
            ReviewSeverity::Warning => "warning",
            ReviewSeverity::Error => "error",
            ReviewSeverity::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReviewFinding {
    pub severity: ReviewSeverity,
    pub category: String,
    pub message: String,
    pub location: Option<String>,
    pub suggestion: Option<String>,
}

impl ReviewFinding {
    pub fn info(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self { severity: ReviewSeverity::Info, category: category.into(), message: message.into(), location: None, suggestion: None }
    }

    pub fn warning(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self { severity: ReviewSeverity::Warning, category: category.into(), message: message.into(), location: None, suggestion: None }
    }

    pub fn error(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self { severity: ReviewSeverity::Error, category: category.into(), message: message.into(), location: None, suggestion: None }
    }

    pub fn critical(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self { severity: ReviewSeverity::Critical, category: category.into(), message: message.into(), location: None, suggestion: None }
    }

    pub fn at(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ReviewTask {
    pub id: String,
    pub subject: String,
    pub context: String,
    pub instructions: Vec<String>,
}

impl ReviewTask {
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            id: format!("review_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()),
            subject: subject.into(),
            context: String::new(),
            instructions: Vec::new(),
        }
    }

    pub fn with_context(mut self, ctx: impl Into<String>) -> Self {
        self.context = ctx.into();
        self
    }

    pub fn with_instruction(mut self, inst: impl Into<String>) -> Self {
        self.instructions.push(inst.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct ReviewResult {
    pub task_id: String,
    pub findings: Vec<ReviewFinding>,
    pub summary: String,
    pub passed: bool,
}

impl ReviewResult {
    pub fn new(task_id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self { task_id: task_id.into(), findings: Vec::new(), summary: summary.into(), passed: true }
    }

    pub fn add_finding(mut self, finding: ReviewFinding) -> Self {
        if finding.severity >= ReviewSeverity::Error {
            self.passed = false;
        }
        self.findings.push(finding);
        self
    }

    pub fn critical_count(&self) -> usize {
        self.findings.iter().filter(|f| f.severity == ReviewSeverity::Critical).count()
    }

    pub fn error_count(&self) -> usize {
        self.findings.iter().filter(|f| f.severity == ReviewSeverity::Error).count()
    }

    pub fn warning_count(&self) -> usize {
        self.findings.iter().filter(|f| f.severity == ReviewSeverity::Warning).count()
    }

    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# Review: {}\n\n", self.task_id));
        out.push_str(&format!("**Summary**: {}\n\n", self.summary));
        out.push_str(&format!("**Status**: {}\n\n", if self.passed { "PASSED" } else { "FAILED" }));
        if !self.findings.is_empty() {
            out.push_str("## Findings\n\n");
            for f in &self.findings {
                let loc = f.location.as_deref().map(|l| format!(" at `{l}`")).unwrap_or_default();
                out.push_str(&format!("- **[{}]** {}: {}{}\n", f.severity.as_str().to_uppercase(), f.category, f.message, loc));
                if let Some(s) = &f.suggestion {
                    out.push_str(&format!("  - Suggestion: {s}\n"));
                }
            }
        }
        out
    }
}

pub fn run_review(task: &ReviewTask, provider_output: &str) -> ReviewResult {
    let result = ReviewResult::new(task.id.clone(), format!("Agent review of: {}", task.subject));
    if provider_output.contains("CRITICAL") || provider_output.contains("critical") {
        result.add_finding(ReviewFinding::critical("agent", "Critical issue detected in review output"))
    } else if provider_output.contains("ERROR") || provider_output.contains("error") {
        result.add_finding(ReviewFinding::error("agent", "Error detected in review output"))
    } else {
        result.add_finding(ReviewFinding::info("agent", "Review completed without critical findings"))
    }
}
