pub mod capture;
pub mod diff;
pub mod audit;
pub mod report;

pub use capture::{Screenshot, ScreenshotConfig, CaptureResult};
pub use diff::{VisualDiff, DiffResult, DiffRegion};
pub use audit::{VisualAudit, AuditCheck, AuditResult};
pub use report::{VisualReport, ReportSection, ReportStatus};
