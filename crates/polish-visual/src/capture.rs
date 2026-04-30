#[derive(Debug, Clone)]
pub struct ScreenshotConfig {
    pub width: u32,
    pub height: u32,
    pub device_pixel_ratio: f32,
    pub full_page: bool,
    pub output_path: String,
}

impl Default for ScreenshotConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            device_pixel_ratio: 1.0,
            full_page: true,
            output_path: "screenshots".into(),
        }
    }
}

impl ScreenshotConfig {
    pub fn mobile() -> Self {
        Self { width: 390, height: 844, device_pixel_ratio: 3.0, full_page: true, output_path: "screenshots".into() }
    }

    pub fn tablet() -> Self {
        Self { width: 768, height: 1024, device_pixel_ratio: 2.0, full_page: true, output_path: "screenshots".into() }
    }
}

#[derive(Debug, Clone)]
pub struct Screenshot {
    pub name: String,
    pub url: String,
    pub config: ScreenshotConfig,
    pub path: Option<String>,
    pub width_px: u32,
    pub height_px: u32,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct CaptureResult {
    pub screenshots: Vec<Screenshot>,
    pub errors: Vec<String>,
    pub duration_ms: u64,
}

impl CaptureResult {
    pub fn empty() -> Self {
        Self { screenshots: Vec::new(), errors: Vec::new(), duration_ms: 0 }
    }

    pub fn add_screenshot(&mut self, s: Screenshot) {
        self.screenshots.push(s);
    }

    pub fn add_error(&mut self, e: impl Into<String>) {
        self.errors.push(e.into());
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn screenshot_count(&self) -> usize {
        self.screenshots.len()
    }
}

pub fn capture_stub(url: &str, name: &str, config: ScreenshotConfig) -> CaptureResult {
    let mut result = CaptureResult::empty();
    result.add_screenshot(Screenshot {
        name: name.to_string(),
        url: url.to_string(),
        path: Some(format!("{}/{}.png", config.output_path, name)),
        width_px: config.width,
        height_px: config.height,
        bytes: Vec::new(),
        config,
    });
    result
}
