//! Headless Chrome/Chromium driver for real captures and rendered-DOM dumps.
//! Fail-closed: every function returns an error when the browser is missing,
//! the page is unreachable, or the output file was not actually written.

use anyhow::{anyhow, bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// Locate a Chrome/Chromium binary on this host.
pub fn find_chrome() -> Result<String> {
    let candidates = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "google-chrome",
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "chrome",
    ];
    for c in candidates {
        if c.starts_with('/') {
            if Path::new(c).exists() {
                return Ok(c.to_string());
            }
        } else if Command::new("which")
            .arg(c)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Ok(c.to_string());
        }
    }
    Err(anyhow!(
        "no Chrome/Chromium binary found — install Chrome or chromium (e.g. `apt install chromium-browser` / Chrome.app)"
    ))
}

fn run_chrome(args: &[&str]) -> Result<std::process::Output> {
    let chrome = find_chrome()?;
    Command::new(&chrome)
        .args([
            "--headless=new",
            "--disable-gpu",
            "--no-sandbox",
            "--hide-scrollbars",
            "--virtual-time-budget=10000",
            "--timeout=30000",
        ])
        .args(args)
        .output()
        .with_context(|| format!("failed to launch {chrome}"))
}

/// Capture a real screenshot of `url` at the given viewport. Verifies the file exists and is non-empty.
pub fn capture(url: &str, width: u32, height: u32, out_path: &str) -> Result<()> {
    if let Some(parent) = Path::new(out_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let size = format!("--window-size={width},{height}");
    let shot = format!("--screenshot={out_path}");
    let out = run_chrome(&[size.as_str(), shot.as_str(), url])?;
    let meta = std::fs::metadata(out_path).map_err(|_| {
        anyhow!(
            "chrome exited but wrote no screenshot for {url}: {}",
            String::from_utf8_lossy(&out.stderr).lines().last().unwrap_or("unknown error")
        )
    })?;
    if meta.len() == 0 {
        bail!("screenshot file is empty for {url}");
    }
    Ok(())
}

/// Return the rendered DOM of `url`. Errors if the page is unreachable or renders empty.
pub fn dump_dom(url: &str) -> Result<String> {
    let out = run_chrome(&["--dump-dom", url])?;
    let dom = String::from_utf8_lossy(&out.stdout).to_string();
    if dom.trim().is_empty() || dom.len() < 64 {
        bail!(
            "page unreachable or rendered empty: {url} ({})",
            String::from_utf8_lossy(&out.stderr).lines().last().unwrap_or("no error output")
        );
    }
    Ok(dom)
}
