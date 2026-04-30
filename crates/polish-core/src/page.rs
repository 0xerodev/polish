use crate::html::HtmlWriter;
use crate::render::Render;
use crate::context::RenderContext;

/// Top-level page model. Renders a full HTML document.
pub struct Page {
    pub meta: PageMeta,
    pub head_extra: Vec<String>,
    pub body: Box<dyn Render>,
}

#[derive(Clone, Default)]
pub struct PageMeta {
    pub title: String,
    pub description: String,
    pub lang: String,
    pub theme_class: String,
    pub charset: String,
    pub viewport: String,
    pub csp: Option<String>,
    pub canonical: Option<String>,
}

impl PageMeta {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            lang: "en".into(),
            charset: "utf-8".into(),
            viewport: "width=device-width, initial-scale=1".into(),
            ..Default::default()
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into(); self
    }

    pub fn with_theme(mut self, cls: impl Into<String>) -> Self {
        self.theme_class = cls.into(); self
    }

    pub fn with_lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = lang.into(); self
    }

    pub fn with_csp(mut self, csp: impl Into<String>) -> Self {
        self.csp = Some(csp.into()); self
    }

    pub fn with_canonical(mut self, url: impl Into<String>) -> Self {
        self.canonical = Some(url.into()); self
    }
}

impl Page {
    pub fn new(meta: PageMeta, body: impl Render + 'static) -> Self {
        Self { meta, head_extra: Vec::new(), body: Box::new(body) }
    }

    pub fn with_head(mut self, html: impl Into<String>) -> Self {
        self.head_extra.push(html.into()); self
    }

    pub fn render_full(&self, ctx: &RenderContext) -> String {
        let mut w = HtmlWriter::with_capacity(16384);
        w.raw("<!DOCTYPE html>");
        w.nl();
        w.open_tag_start("html");
        let lang = if self.meta.lang.is_empty() { "en" } else { &self.meta.lang };
        w.attr("lang", lang);
        if !self.meta.theme_class.is_empty() {
            w.attr("class", &self.meta.theme_class);
        }
        w.tag_end("html");
        w.nl();

        // <head>
        w.open_tag_start("head");
        w.tag_end("head");
        w.nl();
        w.open_tag_start("meta");
        w.attr("charset", &self.meta.charset);
        w.tag_self_close();
        w.nl();
        w.open_tag_start("meta");
        w.attr("name", "viewport");
        w.attr("content", &self.meta.viewport);
        w.tag_self_close();
        w.nl();
        if !self.meta.description.is_empty() {
            w.open_tag_start("meta");
            w.attr("name", "description");
            w.attr("content", &self.meta.description);
            w.tag_self_close();
            w.nl();
        }
        if let Some(url) = &self.meta.canonical {
            w.open_tag_start("link");
            w.attr("rel", "canonical");
            w.attr("href", url);
            w.tag_self_close();
            w.nl();
        }
        w.open("title");
        w.text(&self.meta.title);
        w.close("title");
        w.nl();
        for extra in &self.head_extra {
            w.raw(extra);
            w.nl();
        }
        w.close("head");
        w.nl();

        // <body>
        let body_class = format!("p-body {}", self.meta.theme_class);
        w.open_tag_start("body");
        w.attr("class", body_class.trim());
        w.tag_end("body");
        w.nl();
        self.body.render(&mut w, ctx);
        w.nl();
        w.close("body");
        w.nl();
        w.close("html");
        w.finish()
    }
}

/// A named content slot for layout composition.
pub struct Slot {
    pub name: &'static str,
    content: Option<Box<dyn Render>>,
}

impl Slot {
    pub fn new(name: &'static str) -> Self { Self { name, content: None } }

    pub fn fill(mut self, content: impl Render + 'static) -> Self {
        self.content = Some(Box::new(content)); self
    }

    pub fn is_filled(&self) -> bool { self.content.is_some() }
}

impl Render for Slot {
    fn render(&self, out: &mut HtmlWriter, ctx: &RenderContext) {
        if let Some(c) = &self.content {
            c.render(out, ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TextBody(String);
    impl Render for TextBody {
        fn render(&self, out: &mut HtmlWriter, _ctx: &RenderContext) {
            out.open("main");
            out.text(&self.0);
            out.close("main");
        }
    }

    #[test]
    fn page_renders_full_document() {
        let meta = PageMeta::new("Test Page");
        let page = Page::new(meta, TextBody("Hello, World!".into()));
        let html = page.render_full(&RenderContext::default());
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<title>Test Page</title>"));
        assert!(html.contains("<main>Hello, World!</main>"));
        assert!(html.contains("</html>"));
    }
}
