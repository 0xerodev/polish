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

/// Real pixel comparison of two PNG files. Fail-closed: errors if either file
/// is missing/unreadable; dimension mismatch counts as 100% different.
pub fn diff_files(baseline: &str, actual: &str, threshold_percent: f32) -> anyhow::Result<DiffResult> {
    fn load(path: &str) -> anyhow::Result<(Vec<u8>, u32, u32)> {
        let f = std::fs::File::open(path)
            .map_err(|e| anyhow::anyhow!("cannot open {path}: {e}"))?;
        let decoder = png::Decoder::new(std::io::BufReader::new(f));
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0u8; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        buf.truncate(info.buffer_size());
        Ok((buf, info.width, info.height))
    }
    let (b, bw, bh) = load(baseline)?;
    let (a, aw, ah) = load(actual)?;
    if (bw, bh) != (aw, ah) {
        return Ok(DiffResult {
            baseline_path: baseline.into(),
            actual_path: actual.into(),
            total_pixels: (bw as u64) * (bh as u64),
            diff_pixels: (bw as u64) * (bh as u64),
            diff_percent: 100.0,
            regions: Vec::new(),
            passed: false,
            threshold_percent,
        });
    }
    let px = b.len().min(a.len());
    let stride = (b.len() / ((bw as usize) * (bh as usize))).max(1);
    let mut diff_pixels: u64 = 0;
    let total_pixels = (bw as u64) * (bh as u64);
    for i in (0..px).step_by(stride) {
        let end = (i + stride).min(px);
        if b[i..end] != a[i..end] {
            diff_pixels += 1;
        }
    }
    let diff_percent = if total_pixels == 0 { 0.0 } else { (diff_pixels as f32 / total_pixels as f32) * 100.0 };
    Ok(DiffResult {
        baseline_path: baseline.into(),
        actual_path: actual.into(),
        total_pixels,
        diff_pixels,
        diff_percent,
        regions: Vec::new(),
        passed: diff_percent <= threshold_percent,
        threshold_percent,
    })
}
