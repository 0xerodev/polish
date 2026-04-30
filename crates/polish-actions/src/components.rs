use polish_core::{HtmlWriter, RenderContext, Render};
use crate::form::Form;
use crate::field::{FieldType, FieldMeta};
use crate::csrf::CsrfToken;

/// Renders a complete Form element with all fields, errors, and CSRF.
pub struct FormComponent<'a> {
    pub form: &'a Form,
    pub csrf_token: Option<&'a CsrfToken>,
    pub submit_label: &'a str,
}

impl<'a> Render for FormComponent<'a> {
    fn render(&self, out: &mut HtmlWriter, ctx: &RenderContext) {
        out.open_tag_start("form");
        out.attr("method", &self.form.method);
        out.attr("action", &self.form.action);
        out.attr("class", "p-form");
        out.attr("novalidate", "novalidate");
        out.tag_end("form");

        // CSRF hidden input
        if let Some(csrf) = self.csrf_token {
            out.open_tag_start("input");
            out.attr("type", "hidden");
            out.attr("name", "_csrf");
            out.attr("value", &csrf.value);
            out.tag_self_close();
        }

        // Global error
        if let Some(err) = &self.form.global_error {
            out.open_tag_start("div");
            out.attr("class", "p-result-strip p-err");
            out.attr("role", "alert");
            out.tag_end("div");
            out.text(err);
            out.close("div");
        }

        // Success
        if let Some(ok) = &self.form.success {
            out.open_tag_start("div");
            out.attr("class", "p-result-strip p-ok");
            out.attr("role", "status");
            out.tag_end("div");
            out.text(ok);
            out.close("div");
        }

        // Fields
        for meta in &self.form.fields {
            render_field(out, meta, self.form, ctx);
        }

        // Submit
        out.open_tag_start("button");
        out.attr("type", "submit");
        out.attr("class", "p-btn p-btn-primary");
        out.tag_end("button");
        out.text(self.submit_label);
        out.close("button");

        out.close("form");
    }
}

fn render_field(out: &mut HtmlWriter, meta: &FieldMeta, form: &Form, _ctx: &RenderContext) {
    let value = form.get_value(&meta.name);
    let errors = form.get_errors(&meta.name);
    let has_error = !errors.is_empty();

    // Wrapper
    out.open_tag_start("div");
    out.attr("class", "p-field");
    out.tag_end("div");

    // Label (not for hidden)
    if !matches!(meta.field_type, FieldType::Hidden) {
        out.open_tag_start("label");
        out.attr("for", &format!("f-{}", meta.name));
        out.attr("class", "p-label");
        out.tag_end("label");
        out.text(&meta.label);
        if meta.required {
            out.open_tag_start("span");
            out.attr("class", "p-required");
            out.attr("aria-hidden", "true");
            out.tag_end("span");
            out.raw(" *");
            out.close("span");
        }
        out.close("label");
    }

    let mut input_class = String::from("p-input");
    if has_error { input_class.push_str(" p-error"); }

    match &meta.field_type {
        FieldType::Hidden => {
            out.open_tag_start("input");
            out.attr("type", "hidden");
            out.attr("name", &meta.name);
            out.attr("value", value);
            out.tag_self_close();
        }
        FieldType::Textarea { rows } => {
            out.open_tag_start("textarea");
            out.attr("id", &format!("f-{}", meta.name));
            out.attr("name", &meta.name);
            out.attr("class", &input_class);
            out.attr("rows", &rows.to_string());
            if meta.required { out.attr_bool("required"); }
            if meta.readonly { out.attr_bool("readonly"); }
            if meta.disabled { out.attr_bool("disabled"); }
            if let Some(ph) = &meta.placeholder { out.attr("placeholder", ph); }
            out.tag_end("textarea");
            out.text(value);
            out.close("textarea");
        }
        FieldType::Select { options } => {
            out.open_tag_start("select");
            out.attr("id", &format!("f-{}", meta.name));
            out.attr("name", &meta.name);
            out.attr("class", &input_class);
            if meta.required { out.attr_bool("required"); }
            if meta.disabled { out.attr_bool("disabled"); }
            out.tag_end("select");
            for (opt_val, opt_label) in options {
                out.open_tag_start("option");
                out.attr("value", opt_val);
                if opt_val == value { out.attr_bool("selected"); }
                out.tag_end("option");
                out.text(opt_label);
                out.close("option");
            }
            out.close("select");
        }
        FieldType::Checkbox => {
            out.open_tag_start("input");
            out.attr("type", "checkbox");
            out.attr("id", &format!("f-{}", meta.name));
            out.attr("name", &meta.name);
            out.attr("value", "1");
            if value == "1" || value == "true" { out.attr_bool("checked"); }
            out.tag_self_close();
        }
        _ => {
            let type_str = match &meta.field_type {
                FieldType::Password => "password",
                FieldType::Email => "email",
                FieldType::Number | FieldType::Amount => "number",
                _ => "text",
            };
            out.open_tag_start("input");
            out.attr("type", type_str);
            out.attr("id", &format!("f-{}", meta.name));
            out.attr("name", &meta.name);
            out.attr("class", &input_class);
            out.attr("value", value);
            if meta.required { out.attr_bool("required"); }
            if meta.readonly { out.attr_bool("readonly"); }
            if meta.disabled { out.attr_bool("disabled"); }
            if let Some(ph) = &meta.placeholder { out.attr("placeholder", ph); }
            if let Some(ac) = &meta.autocomplete { out.attr("autocomplete", ac); }
            if has_error { out.attr("aria-invalid", "true"); }
            out.tag_self_close();
        }
    }

    // Field hint
    if let Some(hint) = &meta.hint {
        if !has_error {
            out.open_tag_start("p");
            out.attr("class", "p-field-hint");
            out.tag_end("p");
            out.text(hint);
            out.close("p");
        }
    }

    // Field errors
    for err in errors {
        out.open_tag_start("p");
        out.attr("class", "p-field-error");
        out.attr("role", "alert");
        out.tag_end("p");
        out.text(err);
        out.close("p");
    }

    out.close("div");
}
