use crate::spec::{EndpointDoc, OpenApiSpec};

#[derive(Clone, Debug, Default)]
pub struct DocsSection {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub sub: Vec<DocsSection>,
}

impl DocsSection {
    pub fn new(title: impl Into<String>, slug: impl Into<String>) -> Self {
        Self { title: title.into(), slug: slug.into(), ..Default::default() }
    }
    pub fn content(mut self, c: impl Into<String>) -> Self { self.content = c.into(); self }
    pub fn subsection(mut self, s: DocsSection) -> Self { self.sub.push(s); self }
}

#[derive(Clone, Debug, Default)]
pub struct DocsPage {
    pub title: String,
    pub slug: String,
    pub sections: Vec<DocsSection>,
}

impl DocsPage {
    pub fn new(title: impl Into<String>, slug: impl Into<String>) -> Self {
        Self { title: title.into(), slug: slug.into(), ..Default::default() }
    }
    pub fn section(mut self, s: DocsSection) -> Self { self.sections.push(s); self }
}

#[derive(Clone, Debug, Default)]
pub struct DocsSite {
    pub title: String,
    pub description: String,
    pub version: String,
    pub base_url: String,
    pub pages: Vec<DocsPage>,
    pub spec: Option<OpenApiSpec>,
}

impl DocsSite {
    pub fn new(title: impl Into<String>) -> Self {
        Self { title: title.into(), ..Default::default() }
    }

    pub fn description(mut self, d: impl Into<String>) -> Self { self.description = d.into(); self }
    pub fn version(mut self, v: impl Into<String>) -> Self { self.version = v.into(); self }
    pub fn base_url(mut self, u: impl Into<String>) -> Self { self.base_url = u.into(); self }
    pub fn page(mut self, p: DocsPage) -> Self { self.pages.push(p); self }
    pub fn spec(mut self, s: OpenApiSpec) -> Self { self.spec = Some(s); self }

    pub fn all_endpoints(&self) -> Vec<&EndpointDoc> {
        self.spec.as_ref().map(|s| s.endpoints.iter().collect()).unwrap_or_default()
    }
}
