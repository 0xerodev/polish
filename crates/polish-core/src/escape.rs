/// Escape text for safe insertion into HTML content.
pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    escape_html_into(s, &mut out);
    out
}

pub fn escape_html_into(s: &str, out: &mut String) {
    for ch in s.chars() {
        match ch {
            '&'  => out.push_str("&amp;"),
            '<'  => out.push_str("&lt;"),
            '>'  => out.push_str("&gt;"),
            '"'  => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            '/'  => out.push_str("&#x2F;"),
            _    => out.push(ch),
        }
    }
}

/// Escape a value for use inside an HTML attribute (double-quoted).
pub fn escape_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    escape_attr_into(s, &mut out);
    out
}

pub fn escape_attr_into(s: &str, out: &mut String) {
    for ch in s.chars() {
        match ch {
            '&'  => out.push_str("&amp;"),
            '"'  => out.push_str("&quot;"),
            '<'  => out.push_str("&lt;"),
            '>'  => out.push_str("&gt;"),
            '\'' => out.push_str("&#x27;"),
            _    => out.push(ch),
        }
    }
}

/// Percent-encode a URL component value.
pub fn escape_url(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'
            | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            _ => {
                out.push('%');
                out.push_str(&format!("{:02X}", b));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_escapes_special_chars() {
        assert_eq!(escape_html("<script>alert('xss')</script>"),
            "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;&#x2F;script&gt;");
    }

    #[test]
    fn attr_escapes_quotes() {
        assert_eq!(escape_attr(r#"foo"bar"#), "foo&quot;bar");
    }

    #[test]
    fn url_encodes_spaces() {
        assert_eq!(escape_url("hello world"), "hello%20world");
    }
}
