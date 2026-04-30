use crate::event::EventKind;

#[derive(Debug, Clone)]
pub enum FragmentKind {
    Replace,
    Append,
    Prepend,
    Remove,
}

impl FragmentKind {
    pub fn as_str(&self) -> &str {
        match self {
            FragmentKind::Replace => "replace",
            FragmentKind::Append => "append",
            FragmentKind::Prepend => "prepend",
            FragmentKind::Remove => "remove",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiveFragment {
    pub target_id: String,
    pub kind: FragmentKind,
    pub html: String,
}

impl LiveFragment {
    pub fn replace(target_id: impl Into<String>, html: impl Into<String>) -> Self {
        Self { target_id: target_id.into(), kind: FragmentKind::Replace, html: html.into() }
    }

    pub fn append(target_id: impl Into<String>, html: impl Into<String>) -> Self {
        Self { target_id: target_id.into(), kind: FragmentKind::Append, html: html.into() }
    }

    pub fn prepend(target_id: impl Into<String>, html: impl Into<String>) -> Self {
        Self { target_id: target_id.into(), kind: FragmentKind::Prepend, html: html.into() }
    }

    pub fn remove(target_id: impl Into<String>) -> Self {
        Self { target_id: target_id.into(), kind: FragmentKind::Remove, html: String::new() }
    }

    pub fn to_sse_data(&self) -> String {
        format!(
            r#"{{"target":"{}","op":"{}","html":{}}}"#,
            self.target_id,
            self.kind.as_str(),
            serde_json_str(&self.html)
        )
    }
}

fn serde_json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragment_sse_data() {
        let frag = LiveFragment::replace("main-content", "<p>Updated</p>");
        let data = frag.to_sse_data();
        assert!(data.contains(r#""target":"main-content""#));
        assert!(data.contains(r#""op":"replace""#));
    }
}
