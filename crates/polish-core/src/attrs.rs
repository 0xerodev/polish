use std::collections::BTreeMap;
use crate::html::HtmlWriter;

/// Type-safe attribute builder. Prevents duplicate attributes, enforces escaping.
#[derive(Default, Clone)]
pub struct Attrs {
    map: BTreeMap<String, String>,
    classes: Vec<String>,
    data: Vec<(String, String)>,
    bools: Vec<String>,
}

impl Attrs {
    pub fn new() -> Self { Self::default() }

    pub fn attr(mut self, key: &str, value: impl Into<String>) -> Self {
        self.map.insert(key.to_string(), value.into());
        self
    }

    pub fn class(mut self, cls: &str) -> Self {
        self.classes.push(cls.to_string());
        self
    }

    pub fn classes(mut self, cls: &[&str]) -> Self {
        for c in cls { self.classes.push(c.to_string()); }
        self
    }

    pub fn data(mut self, key: &str, value: impl Into<String>) -> Self {
        self.data.push((key.to_string(), value.into()));
        self
    }

    pub fn bool_attr(mut self, key: &str) -> Self {
        self.bools.push(key.to_string());
        self
    }

    pub fn maybe_attr(self, key: &str, value: Option<impl Into<String>>) -> Self {
        if let Some(v) = value { self.attr(key, v) } else { self }
    }

    pub fn maybe_class(self, cls: &str, condition: bool) -> Self {
        if condition { self.class(cls) } else { self }
    }

    /// Write all attributes into the HtmlWriter (must be called between open_tag_start and tag_end).
    pub fn write_into(&self, w: &mut HtmlWriter) {
        if !self.classes.is_empty() {
            w.attr("class", &self.classes.join(" "));
        }
        for (k, v) in &self.map {
            w.attr(k, v);
        }
        for (k, v) in &self.data {
            w.data_attr(k, v);
        }
        for b in &self.bools {
            w.attr_bool(b);
        }
    }

    pub fn has_class(&self, cls: &str) -> bool {
        self.classes.iter().any(|c| c == cls)
    }
}
