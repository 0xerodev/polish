use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Number,
    Amount,
    Address,
    Email,
    Password,
    Hidden,
    Checkbox,
    Select { options: Vec<(String, String)> },
    Textarea { rows: u32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldMeta {
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
    pub placeholder: Option<String>,
    pub required: bool,
    pub readonly: bool,
    pub disabled: bool,
    pub autocomplete: Option<String>,
    pub hint: Option<String>,
}

impl FieldMeta {
    pub fn new(name: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            field_type: FieldType::Text,
            placeholder: None,
            required: false,
            readonly: false,
            disabled: false,
            autocomplete: None,
            hint: None,
        }
    }

    pub fn required(mut self) -> Self { self.required = true; self }
    pub fn readonly(mut self) -> Self { self.readonly = true; self }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self { self.placeholder = Some(p.into()); self }
    pub fn hint(mut self, h: impl Into<String>) -> Self { self.hint = Some(h.into()); self }
    pub fn autocomplete(mut self, ac: impl Into<String>) -> Self { self.autocomplete = Some(ac.into()); self }
    pub fn field_type(mut self, t: FieldType) -> Self { self.field_type = t; self }
    pub fn number(self) -> Self { self.field_type(FieldType::Number) }
    pub fn password(self) -> Self { self.field_type(FieldType::Password) }
    pub fn email(self) -> Self { self.field_type(FieldType::Email) }
    pub fn hidden(self) -> Self { self.field_type(FieldType::Hidden) }
    pub fn textarea(self, rows: u32) -> Self { self.field_type(FieldType::Textarea { rows }) }
    pub fn select(self, options: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.field_type(FieldType::Select {
            options: options.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
        })
    }
}

/// A runtime form field with value and errors.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Field {
    pub meta: Option<FieldMeta>,
    pub name: String,
    pub value: String,
    pub errors: Vec<String>,
    pub touched: bool,
}

impl Field {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), ..Default::default() }
    }

    pub fn with_value(mut self, v: impl Into<String>) -> Self { self.value = v.into(); self }
    pub fn with_meta(mut self, m: FieldMeta) -> Self { self.meta = Some(m); self }
    pub fn add_error(&mut self, e: impl Into<String>) { self.errors.push(e.into()); }
    pub fn is_valid(&self) -> bool { self.errors.is_empty() }
    pub fn is_empty(&self) -> bool { self.value.trim().is_empty() }
}
