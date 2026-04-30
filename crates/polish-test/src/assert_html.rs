pub fn assert_contains(html: &str, needle: &str) -> Result<(), String> {
    if html.contains(needle) {
        Ok(())
    } else {
        Err(format!("HTML does not contain: {needle:?}\nHTML (first 500 chars): {}", &html[..html.len().min(500)]))
    }
}

pub fn assert_not_contains(html: &str, needle: &str) -> Result<(), String> {
    if !html.contains(needle) {
        Ok(())
    } else {
        let pos = html.find(needle).unwrap();
        Err(format!("HTML unexpectedly contains: {needle:?} at position {pos}"))
    }
}

pub fn assert_attr(html: &str, attr: &str, value: &str) -> Result<(), String> {
    let pattern = format!(r#"{attr}="{value}""#);
    if html.contains(&pattern) {
        Ok(())
    } else {
        Err(format!("HTML does not contain attribute {attr}={value:?}"))
    }
}

pub fn assert_no_script_injection(html: &str) -> Result<(), String> {
    let injection_patterns = [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "onclick=",
        "onmouseover=",
        "eval(",
        "document.cookie",
    ];
    for pattern in injection_patterns {
        if html.to_lowercase().contains(pattern) {
            return Err(format!("Potential script injection detected: {pattern:?} found in HTML"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_xss_script_tag() {
        let html = "<p>Hello &lt;script&gt;alert(1)&lt;/script&gt;</p>";
        assert_no_script_injection(html).unwrap();

        let html_unsafe = "<p>Hello <script>alert(1)</script></p>";
        assert!(assert_no_script_injection(html_unsafe).is_err());
    }

    #[test]
    fn assert_contains_works() {
        assert_contains("<p>Hello</p>", "Hello").unwrap();
        assert!(assert_contains("<p>Hello</p>", "World").is_err());
    }
}
