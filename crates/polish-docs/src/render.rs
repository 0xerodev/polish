use polish_core::{HtmlWriter, RenderContext, Render};
use crate::site::DocsSite;
use crate::spec::EndpointDoc;

struct SidebarNav<'a> {
    site: &'a DocsSite,
    current_slug: &'a str,
}

impl<'a> Render for SidebarNav<'a> {
    fn render(&self, out: &mut HtmlWriter, _ctx: &RenderContext) {
        out.open_tag_start("nav");
        out.attr("class", "p-docs-sidebar");
        out.tag_end("nav");

        out.open_tag_start("div");
        out.attr("class", "p-header-title");
        out.attr("style", "margin-bottom:16px");
        out.tag_end("div");
        out.text(&self.site.title);
        out.close("div");

        if self.site.spec.is_some() {
            render_nav_item(out, "API Reference", "/docs/api", self.current_slug == "api");
        }

        for page in &self.site.pages {
            render_nav_item(out, &page.title, &format!("/docs/{}", page.slug), self.current_slug == page.slug);
        }

        out.close("nav");
    }
}

fn render_nav_item(out: &mut HtmlWriter, label: &str, href: &str, active: bool) {
    out.open_tag_start("a");
    out.attr("href", href);
    let cls = if active { "p-link" } else { "p-muted" };
    out.attr("class", &format!("{} p-nav-item", cls));
    out.attr("style", "display:block;padding:6px 0;text-decoration:none;font-size:13px");
    out.tag_end("a");
    out.text(label);
    out.close("a");
}

struct EndpointBlock<'a> {
    ep: &'a EndpointDoc,
}

impl<'a> Render for EndpointBlock<'a> {
    fn render(&self, out: &mut HtmlWriter, _ctx: &RenderContext) {
        out.open_tag_start("div");
        out.attr("class", "p-endpoint-block");
        out.tag_end("div");

        // Header with method badge and path
        out.open_tag_start("div");
        out.attr("class", "p-endpoint-header");
        out.tag_end("div");

        let method_lc = self.ep.method.to_lowercase();
        out.open_tag_start("span");
        out.attr("class", &format!("p-method-pill p-method-{}", method_lc));
        out.tag_end("span");
        out.text(&self.ep.method.to_uppercase());
        out.close("span");

        out.open_tag_start("code");
        out.attr("class", "p-mono");
        out.tag_end("code");
        out.text(&self.ep.path);
        out.close("code");

        if self.ep.auth_required {
            out.open_tag_start("span");
            out.attr("class", "p-pill p-pill-warn");
            out.attr("style", "margin-left:auto");
            out.tag_end("span");
            out.text("auth required");
            out.close("span");
        }

        out.close("div");

        // Description
        out.open_tag_start("div");
        out.attr("style", "padding:12px 16px");
        out.tag_end("div");

        out.open_tag_start("p");
        out.attr("style", "font-weight:600;margin-bottom:4px");
        out.tag_end("p");
        out.text(&self.ep.summary);
        out.close("p");

        if !self.ep.description.is_empty() {
            out.open("p");
            out.text(&self.ep.description);
            out.close("p");
        }

        if !self.ep.params.is_empty() {
            out.open_tag_start("h4");
            out.attr("style", "margin:12px 0 6px");
            out.tag_end("h4");
            out.text("Parameters");
            out.close("h4");

            out.open_tag_start("table");
            out.attr("class", "p-table");
            out.tag_end("table");
            out.open("thead");
            out.open("tr");
            for th in &["Name", "In", "Type", "Required", "Description"] {
                out.open("th"); out.text(th); out.close("th");
            }
            out.close("tr");
            out.close("thead");
            out.open("tbody");
            for p in &self.ep.params {
                out.open("tr");
                out.open_tag_start("td");
                out.attr("class", "p-mono");
                out.tag_end("td");
                out.text(&p.name);
                out.close("td");
                out.open("td"); out.text(&p.location); out.close("td");
                out.open_tag_start("td");
                out.attr("class", "p-mono");
                out.tag_end("td");
                out.text(&p.schema_type);
                out.close("td");
                out.open("td"); out.text(if p.required { "yes" } else { "no" }); out.close("td");
                out.open("td"); out.text(&p.description); out.close("td");
                out.close("tr");
            }
            out.close("tbody");
            out.close("table");
        }

        if !self.ep.responses.is_empty() {
            out.open_tag_start("h4");
            out.attr("style", "margin:12px 0 6px");
            out.tag_end("h4");
            out.text("Responses");
            out.close("h4");

            for r in &self.ep.responses {
                out.open_tag_start("div");
                out.attr("class", "p-result-strip");
                let _cls = if r.status < 300 { "p-ok" } else { "p-err" };
                out.attr("style", if r.status < 300 { "background:rgba(34,197,94,0.06);border-left-color:#22c55e" } else { "background:rgba(239,68,68,0.06);border-left-color:#ef4444" });
                out.tag_end("div");
                out.open_tag_start("strong");
                out.attr("class", "p-mono");
                out.tag_end("strong");
                out.text(&r.status.to_string());
                out.close("strong");
                out.text(" — ");
                out.text(&r.description);
                out.close("div");
            }
        }

        if let Some(ex) = &self.ep.example_request {
            out.open_tag_start("h4");
            out.attr("style", "margin:12px 0 6px");
            out.tag_end("h4");
            out.text("Example");
            out.close("h4");

            out.open_tag_start("pre");
            out.attr("class", "p-code");
            out.tag_end("pre");
            out.text(&serde_json::to_string_pretty(ex).unwrap_or_default());
            out.close("pre");
        }

        out.close("div");
        out.close("div");
    }
}

pub fn render_docs_page(site: &DocsSite, current_slug: &str, ctx: &RenderContext) -> String {
    let mut w = HtmlWriter::with_capacity(32768);

    w.open_tag_start("div");
    w.attr("class", "p-docs-layout");
    w.tag_end("div");

    // Sidebar
    SidebarNav { site, current_slug }.render(&mut w, ctx);

    // Main content
    w.open_tag_start("main");
    w.attr("class", "p-docs-content");
    w.tag_end("main");

    if current_slug == "api" {
        if let Some(spec) = &site.spec {
            w.open_tag_start("h1");
            w.attr("style", "margin-bottom:24px");
            w.tag_end("h1");
            w.text("API Reference");
            w.close("h1");

            for ep in &spec.endpoints {
                EndpointBlock { ep }.render(&mut w, ctx);
            }
        }
    } else if let Some(page) = site.pages.iter().find(|p| p.slug == current_slug) {
        w.open_tag_start("h1");
        w.attr("style", "margin-bottom:24px");
        w.tag_end("h1");
        w.text(&page.title);
        w.close("h1");

        for section in &page.sections {
            w.open_tag_start("section");
            w.attr("class", "p-section");
            w.tag_end("section");

            w.open("h2");
            w.text(&section.title);
            w.close("h2");

            if !section.content.is_empty() {
                w.open("p");
                w.text(&section.content);
                w.close("p");
            }

            w.close("section");
        }
    }

    w.close("main");
    w.close("div");
    w.finish()
}
