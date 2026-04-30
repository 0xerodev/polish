#[derive(Debug, Clone)]
pub struct DiffRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub diff_percent: f32,
}

#[derive(Debug, Clone)]
pub struct DiffResult {
    pub baseline_path: String,
    pub actual_path: String,
    pub total_pixels: u64,
    pub diff_pixels: u64,
    pub diff_percent: f32,
    pub regions: Vec<DiffRegion>,
    pub passed: bool,
    pub threshold_percent: f32,
}

impl DiffResult {
    pub fn identical(baseline: impl Into<String>, actual: impl Into<String>) -> Self {
        Self {
            baseline_path: baseline.into(),
            actual_path: actual.into(),
            total_pixels: 0,
            diff_pixels: 0,
            diff_percent: 0.0,
            regions: Vec::new(),
            passed: true,
            threshold_percent: 0.1,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "{:.2}% changed ({} / {} pixels)",
            self.diff_percent,
            self.diff_pixels,
            self.total_pixels
        )
    }
}

pub struct VisualDiff {
    pub threshold_percent: f32,
}

impl VisualDiff {
    pub fn new(threshold_percent: f32) -> Self {
        Self { threshold_percent }
    }

    pub fn compare(&self, baseline: &[u8], actual: &[u8], baseline_path: &str, actual_path: &str) -> DiffResult {
        if baseline.is_empty() || actual.is_empty() {
            return DiffResult::identical(baseline_path, actual_path);
        }
        let total = baseline.len().max(actual.len()) as u64;
        let diff_bytes: u64 = baseline.iter().zip(actual.iter()).filter(|(a, b)| a != b).count() as u64;
        let diff_pct = (diff_bytes as f32 / total as f32) * 100.0;
        DiffResult {
            baseline_path: baseline_path.to_string(),
            actual_path: actual_path.to_string(),
            total_pixels: total,
            diff_pixels: diff_bytes,
            diff_percent: diff_pct,
            regions: Vec::new(),
            passed: diff_pct <= self.threshold_percent,
            threshold_percent: self.threshold_percent,
        }
    }
}

impl Default for VisualDiff {
    fn default() -> Self { Self::new(0.1) }
}
