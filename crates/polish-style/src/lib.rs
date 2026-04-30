pub mod tokens;
pub mod theme;
pub mod css;
pub mod components;

pub use tokens::{ColorPalette, Spacing, Radius, Typography, Motion, Breakpoints, Shadow};
pub use theme::{Theme, ThemeMode, BuiltinTheme};
pub use css::{CssWriter, StyleSheet, ClassName};

/// Render a complete HTML page with the given theme CSS and body HTML injected inline.
pub fn render_themed_page(title: &str, body_html: &str, theme: &crate::theme::Theme) -> String {
    let css = StyleSheet::generate(theme);
    let mut out = String::with_capacity(css.css.len() + body_html.len() + 2048);
    out.push_str("<!DOCTYPE html>\n<html lang=\"en\" class=\"");
    out.push_str(&css.theme_class);
    out.push_str("\">\n<head>\n<meta charset=\"utf-8\">\n");
    out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    out.push_str("<title>");
    for c in title.chars() {
        match c { '<' => out.push_str("&lt;"), '>' => out.push_str("&gt;"), '&' => out.push_str("&amp;"), _ => out.push(c) }
    }
    out.push_str("</title>\n<style>\n");
    out.push_str(&css.css);
    out.push_str("\n</style>\n</head>\n<body class=\"p-body ");
    out.push_str(&css.theme_class);
    out.push_str("\">\n");
    out.push_str(body_html);
    out.push_str("\n</body>\n</html>");
    out
}
