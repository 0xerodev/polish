use crate::html::{HtmlWriter, SafeHtml};
use crate::context::RenderContext;

/// The fundamental rendering interface. All renderable types implement this.
pub trait Render {
    fn render(&self, out: &mut HtmlWriter, ctx: &RenderContext);

    /// Convenience: render to a String.
    fn to_html(&self, ctx: &RenderContext) -> String {
        let mut w = HtmlWriter::new();
        self.render(&mut w, ctx);
        w.finish()
    }

    /// Convenience: render to SafeHtml (output of Render is always safe).
    fn to_safe_html(&self, ctx: &RenderContext) -> SafeHtml {
        SafeHtml::from_rendered(self.to_html(ctx))
    }
}

/// Extends Render with component metadata.
pub trait Component: Render {
    fn component_name(&self) -> &'static str;
    fn is_visible(&self) -> bool { true }
}

/// A lazy sequence of renderable items.
pub struct Fragment {
    items: Vec<Box<dyn Render>>,
}

impl Fragment {
    pub fn new() -> Self { Self { items: Vec::new() } }

    pub fn push(&mut self, item: impl Render + 'static) {
        self.items.push(Box::new(item));
    }

    pub fn with(mut self, item: impl Render + 'static) -> Self {
        self.items.push(Box::new(item));
        self
    }

    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    pub fn len(&self) -> usize { self.items.len() }
}

impl Default for Fragment {
    fn default() -> Self { Self::new() }
}

impl Render for Fragment {
    fn render(&self, out: &mut HtmlWriter, ctx: &RenderContext) {
        for item in &self.items {
            item.render(out, ctx);
        }
    }
}

/// Implement Render for SafeHtml (pass-through — already safe).
impl Render for SafeHtml {
    fn render(&self, out: &mut HtmlWriter, _ctx: &RenderContext) {
        out.raw(self.as_str());
    }
}

/// Implement Render for &str (escapes content).
impl Render for &str {
    fn render(&self, out: &mut HtmlWriter, _ctx: &RenderContext) {
        out.text(self);
    }
}

/// Implement Render for String (escapes content).
impl Render for String {
    fn render(&self, out: &mut HtmlWriter, _ctx: &RenderContext) {
        out.text(self);
    }
}

/// Extension trait for ergonomic rendering.
pub trait RenderExt: Render {
    fn render_to_string(&self) -> String {
        self.to_html(&RenderContext::default())
    }
}

impl<T: Render> RenderExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    struct Span { text: String }
    impl Render for Span {
        fn render(&self, out: &mut HtmlWriter, _ctx: &RenderContext) {
            out.open_tag_start("span");
            out.tag_end("span");
            out.text(&self.text);
            out.close("span");
        }
    }

    #[test]
    fn fragment_renders_items() {
        let mut f = Fragment::new();
        f.push(Span { text: "hello".into() });
        f.push(Span { text: "world".into() });
        let html = f.render_to_string();
        assert!(html.contains("<span>hello</span>"));
        assert!(html.contains("<span>world</span>"));
    }

    #[test]
    fn str_render_escapes() {
        let s = "<danger>";
        let html = s.render_to_string();
        assert_eq!(html, "&lt;danger&gt;");
    }
}
