pub mod matrix;
pub mod surface;
pub mod leakage;

pub use matrix::{CapabilityMatrix, CapabilityRow, CapabilityEntry};
pub use surface::{Surface, SurfacePolicy};
pub use leakage::{LeakageReport, LeakageViolation, scan_html_for_leakage};
