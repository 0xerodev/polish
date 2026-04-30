use polish_core::{HtmlWriter, RenderContext, Render};

/// Renders a full <style> tag containing theme CSS.
pub struct StyleTag {
    pub css: String,
    pub nonce: Option<String>,
}

impl Render for StyleTag {
    fn render(&self, out: &mut HtmlWriter, _ctx: &RenderContext) {
        out.open_tag_start("style");
        if let Some(n) = &self.nonce {
            out.attr("nonce", n);
        }
        out.tag_end("style");
        out.raw(&self.css);
        out.close("style");
    }
}

/// Renders a Google Fonts preconnect + load snippet.
pub struct FontPreload;

impl Render for FontPreload {
    fn render(&self, out: &mut HtmlWriter, _ctx: &RenderContext) {
        out.raw(r#"<link rel="preconnect" href="https://fonts.googleapis.com" />"#);
        out.raw(r#"<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />"#);
        out.raw(r#"<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet" />"#);
    }
}
