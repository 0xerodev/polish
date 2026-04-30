use crate::escape::{escape_html_into, escape_attr_into};

/// Core HTML output buffer. All text content is escaped by default.
/// Raw HTML injection requires explicit SafeHtml.
pub struct HtmlWriter {
    buf: String,
    open_tags: Vec<&'static str>,
}

impl HtmlWriter {
    pub fn new() -> Self {
        Self { buf: String::with_capacity(4096), open_tags: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self { buf: String::with_capacity(cap), open_tags: Vec::new() }
    }

    /// Write raw trusted HTML — only use with SafeHtml or known-safe content.
    #[inline]
    pub fn raw(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    /// Write HTML-escaped text content.
    #[inline]
    pub fn text(&mut self, s: &str) {
        escape_html_into(s, &mut self.buf);
    }

    /// Open a tag. Must be closed with `close()`.
    pub fn open(&mut self, tag: &'static str) {
        self.buf.push('<');
        self.buf.push_str(tag);
        self.buf.push('>');
        self.open_tags.push(tag);
    }

    /// Open a tag with attributes (call attr() after open_tag_start, then tag_end).
    pub fn open_tag_start(&mut self, tag: &str) {
        self.buf.push('<');
        self.buf.push_str(tag);
    }

    /// Write an attribute. Must be called between open_tag_start and tag_end.
    pub fn attr(&mut self, name: &str, value: &str) {
        self.buf.push(' ');
        self.buf.push_str(name);
        self.buf.push_str("=\"");
        escape_attr_into(value, &mut self.buf);
        self.buf.push('"');
    }

    /// Write a boolean attribute (e.g. disabled, checked).
    pub fn attr_bool(&mut self, name: &str) {
        self.buf.push(' ');
        self.buf.push_str(name);
    }

    /// Write a data attribute.
    pub fn data_attr(&mut self, key: &str, value: &str) {
        self.buf.push_str(" data-");
        self.buf.push_str(key);
        self.buf.push_str("=\"");
        escape_attr_into(value, &mut self.buf);
        self.buf.push('"');
    }

    /// Finish opening tag (close the `<tag ...` part).
    pub fn tag_end(&mut self, tag: &'static str) {
        self.buf.push('>');
        self.open_tags.push(tag);
    }

    /// Self-closing tag end (for void elements or explicit self-close).
    pub fn tag_self_close(&mut self) {
        self.buf.push_str(" />");
    }

    /// Close the most recently opened tag.
    pub fn close(&mut self, tag: &str) {
        self.buf.push_str("</");
        self.buf.push_str(tag);
        self.buf.push('>');
        // pop matching tag from stack
        if let Some(pos) = self.open_tags.iter().rposition(|&t| t == tag) {
            self.open_tags.remove(pos);
        }
    }

    /// Void element (no closing tag): <tag attrs />
    pub fn void_element(&mut self, tag: &str, attrs: &[(&str, &str)]) {
        self.buf.push('<');
        self.buf.push_str(tag);
        for (k, v) in attrs {
            self.buf.push(' ');
            self.buf.push_str(k);
            self.buf.push_str("=\"");
            escape_attr_into(v, &mut self.buf);
            self.buf.push('"');
        }
        self.buf.push_str(" />");
    }

    /// Write an HTML comment (content is not escaped, caller must ensure safety).
    pub fn comment(&mut self, text: &str) {
        self.buf.push_str("<!-- ");
        self.buf.push_str(text);
        self.buf.push_str(" -->");
    }

    /// Write a newline (for readability in debug output).
    #[inline]
    pub fn nl(&mut self) {
        self.buf.push('\n');
    }

    /// Consume writer and return the HTML string.
    pub fn finish(self) -> String {
        self.buf
    }

    /// Borrow the buffer (for appending sub-renders).
    pub fn as_str(&self) -> &str {
        &self.buf
    }

    /// Append from another writer.
    pub fn append(&mut self, other: HtmlWriter) {
        self.buf.push_str(&other.buf);
    }
}

impl Default for HtmlWriter {
    fn default() -> Self { Self::new() }
}

/// Pre-escaped / trusted HTML fragment. Creation is unsafe-annotated to force
/// caller awareness. The only safe constructors are from already-escaped sources.
#[derive(Clone, Default)]
pub struct SafeHtml(pub(crate) String);

impl SafeHtml {
    /// Create from a known-safe string (e.g. from Render::to_html()).
    /// SAFETY: caller guarantees string contains no unescaped user input.
    pub fn from_rendered(s: String) -> Self { Self(s) }

    pub fn empty() -> Self { Self(String::new()) }

    pub fn as_str(&self) -> &str { &self.0 }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn push_str(&mut self, s: &SafeHtml) {
        self.0.push_str(&s.0);
    }
}

impl std::fmt::Display for SafeHtml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_is_escaped() {
        let mut w = HtmlWriter::new();
        w.open("p");
        w.text("<script>xss</script>");
        w.close("p");
        assert_eq!(w.finish(), "<p>&lt;script&gt;xss&lt;&#x2F;script&gt;</p>");
    }

    #[test]
    fn attr_is_escaped() {
        let mut w = HtmlWriter::new();
        w.open_tag_start("input");
        w.attr("value", r#""><img onerror=alert(1)>"#);
        w.tag_self_close();
        let html = w.finish();
        assert!(!html.contains("<img"), "XSS via attr not escaped");
    }

    #[test]
    fn void_element() {
        let mut w = HtmlWriter::new();
        w.void_element("input", &[("type", "text"), ("name", "q")]);
        assert_eq!(w.finish(), r#"<input type="text" name="q" />"#);
    }
}
