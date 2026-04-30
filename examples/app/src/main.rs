//! Polish example server — real axum HTTP, SSE live updates, CSRF form processing.

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use axum::{
    Router,
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
};
use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::StreamExt;
use serde_json::json;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::trace::TraceLayer;

use polish_actions::{
    Action, ActionOutput, CsrfStore, FieldMeta, Form as PolishForm, ParsedForm, Validator,
    execute_pipeline,
};
use polish_actions::components::FormComponent;
use polish_core::{RenderContext, escape_html};
use polish_docs::{DocsSite, DocsPage, DocsSection, EndpointDoc, OpenApiSpec, ResponseDoc};
use polish_live::EventKind;
use polish_server::{LiveBus, ServerConfig};
use polish_style::{BuiltinTheme, StyleSheet};

// ── Shared app state ──────────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    csrf: Arc<CsrfStore>,
    bus: LiveBus,
    openapi_json: Arc<String>,
}

// ── Security headers middleware ───────────────────────────────────────────────

const CSP: &str = "default-src 'self'; script-src 'unsafe-inline'; style-src 'unsafe-inline'";

fn secure_html(status: StatusCode, html: String) -> Response {
    (
        status,
        [
            ("Content-Type", "text/html; charset=utf-8"),
            ("Content-Security-Policy", CSP),
            ("X-Frame-Options", "DENY"),
            ("X-Content-Type-Options", "nosniff"),
            ("Referrer-Policy", "strict-origin-when-cross-origin"),
        ],
        html,
    ).into_response()
}

// ── Client-side fragment-patch JS (no external deps) ─────────────────────────

const LIVE_JS: &str = r#"<script>
(function(){
  var es = new EventSource('/live/events');
  es.addEventListener('fragment', function(e){
    var f = JSON.parse(e.data);
    var el = document.getElementById(f.target);
    if (!el) return;
    // Parse HTML safely via template to avoid innerHTML XSS surface
    var tmpl = document.createElement('template');
    tmpl.innerHTML = f.html;
    if (f.op === 'replace') el.replaceWith(tmpl.content.firstChild || el);
    else if (f.op === 'append')  el.appendChild(tmpl.content.firstChild);
    else if (f.op === 'prepend') el.prepend(tmpl.content.firstChild);
    else if (f.op === 'remove')  el.remove();
  });
  es.addEventListener('heartbeat', function(){ });
  es.onerror = function(){ setTimeout(function(){ location.reload(); }, 3000); };
})();
</script>"#;

// ── Page renderer ─────────────────────────────────────────────────────────────

fn themed_page(title: &str, body_html: &str, include_live: bool) -> String {
    let theme = BuiltinTheme::GlassHud.theme();
    let css = StyleSheet::generate(&theme);
    let live_script = if include_live { LIVE_JS } else { "" };
    format!(r#"<!DOCTYPE html>
<html lang="en" class="{tc}">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title}</title>
  <style>{css}</style>
</head>
<body>
{body}
{live}
</body>
</html>"#,
        tc = css.theme_class,
        css = css.css,
        title = escape_html(title),
        body = body_html,
        live = live_script,
    )
}

// ── Routes ────────────────────────────────────────────────────────────────────

async fn index_handler(State(state): State<AppState>) -> Response {
    let token = state.csrf.issue("anon");
    let form_def = make_form();
    let ctx = RenderContext::default();
    use polish_core::Render;
    let form_html = FormComponent { form: &form_def, csrf_token: Some(&token), submit_label: "Place Order" }
        .to_html(&ctx);
    let body = format!(
        r#"<div class="p-container" style="max-width:520px;margin:4rem auto">
  <div class="p-card">
    <h1 style="margin-bottom:1.5rem">Order Form</h1>
    <div id="result"></div>
    {form}
  </div>
</div>"#,
        form = form_html
    );
    secure_html(StatusCode::OK, themed_page("Order Form — Polish", &body, true))
}

async fn submit_handler(
    State(state): State<AppState>,
    Form(fields): Form<HashMap<String, String>>,
) -> Response {
    let parsed = ParsedForm(fields);
    let validator = make_validator();
    let bus = state.bus.clone();

    let result = execute_pipeline(&parsed, &state.csrf, &validator, |form| {
        let name   = escape_html(form.get_or_empty("name"));
        let amount = escape_html(form.get_or_empty("amount"));
        Ok(Action::replace(format!(
            r#"<div class="p-result-strip p-ok" id="result">
              <strong>Order placed!</strong> {name} — ${amount}
            </div>"#
        )))
    });

    match result {
        Ok(r) => {
            if let ActionOutput::Replace { ref page_html } = r.outcome {
                use polish_live::LiveFragment;
                bus.fragment(LiveFragment::replace("result", page_html.clone()));
            }
            let html = match r.outcome {
                ActionOutput::Replace { page_html } => page_html,
                _ => String::new(),
            };
            secure_html(StatusCode::OK, themed_page("Success — Polish",
                &format!(r#"<div class="p-container" style="max-width:520px;margin:4rem auto">
                  <div class="p-card">{}<a href="/" class="p-btn" style="margin-top:1rem;display:inline-block">Back</a></div>
                </div>"#, html),
                false,
            ))
        }
        Err(e) => {
            let errors = make_validator().validate(&parsed);
            let form_def = make_form().with_errors(errors);
            let csrf_token = state.csrf.issue("anon");
            let ctx = RenderContext::default();
            use polish_core::Render;
            let form_html = FormComponent {
                form: &form_def,
                csrf_token: Some(&csrf_token),
                submit_label: "Place Order",
            }.to_html(&ctx);
            let body = format!(
                r#"<div class="p-container" style="max-width:520px;margin:4rem auto">
                  <div class="p-card">
                    <h1 style="margin-bottom:1.5rem">Order Form</h1>
                    <div class="p-result-strip p-err" style="margin-bottom:1rem">{}</div>
                    <div id="result"></div>
                    {}
                  </div>
                </div>"#,
                escape_html(&e.user_message()), form_html
            );
            secure_html(StatusCode::BAD_REQUEST, themed_page("Error — Polish", &body, true))
        }
    }
}

async fn sse_handler(State(state): State<AppState>) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.bus.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        let event = msg.ok()?;
        let mut e = Event::default().data(event.data);
        if event.kind != EventKind::Message {
            e = e.event(event.kind.as_str().to_string());
        }
        Some(Ok::<Event, Infallible>(e))
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({"ok": true, "service": "polish-example"}))
}

async fn openapi_handler(State(state): State<AppState>) -> impl IntoResponse {
    (
        [
            ("Content-Type", "application/json"),
            ("Cache-Control", "public, max-age=300"),
        ],
        state.openapi_json.as_ref().clone(),
    )
}

async fn docs_handler() -> Response {
    let docs_site = DocsSite::new("Polish Docs")
        .page(DocsPage::new("Getting Started", "getting-started")
            .section(DocsSection::new("Overview", "overview")
                .content("Polish is a Rust-first, server-authoritative frontend platform. The server renders HTML; the browser just displays it."))
            .section(DocsSection::new("Forms", "forms")
                .content("Forms are defined in Rust and validated server-side. No client-side validation JS required. CSRF protection is automatic."))
            .section(DocsSection::new("Live Updates", "live")
                .content("Real-time DOM patches are pushed via SSE. Server calls bus.fragment() to update any element by ID.")))
        .page(DocsPage::new("API Reference", "api-reference")
            .section(DocsSection::new("Actions", "actions")
                .content("execute_pipeline(): CSRF check → validate → handler → ActionResult. One call does it all.")));

    let mut body = String::from(r#"<div class="p-container" style="max-width:780px;margin:4rem auto">"#);
    body.push_str(&format!("<h1>{}</h1>", escape_html(&docs_site.title)));
    for page in &docs_site.pages {
        body.push_str(&format!("<h2>{}</h2>", escape_html(&page.title)));
        for section in &page.sections {
            body.push_str(&format!(
                r#"<div class="p-card" style="margin-bottom:1.5rem"><h3>{}</h3><p>{}</p></div>"#,
                escape_html(&section.title), escape_html(&section.content)
            ));
        }
    }
    body.push_str("</div>");
    secure_html(StatusCode::OK, themed_page("Polish Docs", &body, false))
}

async fn not_found_handler() -> Response {
    secure_html(StatusCode::NOT_FOUND,
        themed_page("Not Found — Polish",
            r#"<div class="p-container" style="max-width:520px;margin:4rem auto">
               <div class="p-card"><h1>404</h1><p>Page not found.</p><a href="/">Home</a></div>
               </div>"#,
            false))
}

// ── Form / validator factories ────────────────────────────────────────────────

fn make_form() -> PolishForm {
    PolishForm::new("order-form", "/submit")
        .field(FieldMeta::new("name", "Full Name").required().placeholder("Alice Smith"))
        .field(FieldMeta::new("email", "Email").required().email().placeholder("alice@example.com"))
        .field(FieldMeta::new("amount", "Amount (USD)").required().number().placeholder("100"))
}

fn make_validator() -> Validator {
    Validator::new()
        .required("name").min_len("name", 2).max_len("name", 100)
        .required("email").email("email")
        .required("amount").numeric("amount").min_value("amount", 1.0).max_value("amount", 10000.0)
}

fn build_openapi() -> String {
    OpenApiSpec::new("Polish Demo API", "0.1.0")
        .server("http://localhost:3000")
        .endpoint(
            EndpointDoc::new("GET", "/", "Render order form")
                .tag("pages")
                .response(ResponseDoc { status: 200, description: "HTML page".into(), schema: None, example: None }),
        )
        .endpoint(
            EndpointDoc::new("POST", "/submit", "Submit order form (CSRF + validation)")
                .tag("actions")
                .response(ResponseDoc { status: 200, description: "Success HTML".into(), schema: None, example: None })
                .response(ResponseDoc { status: 400, description: "Validation error HTML".into(), schema: None, example: None }),
        )
        .endpoint(
            EndpointDoc::new("GET", "/live/events", "SSE live event stream (fragments + heartbeat)")
                .tag("live")
                .response(ResponseDoc { status: 200, description: "text/event-stream".into(), schema: None, example: None }),
        )
        .endpoint(
            EndpointDoc::new("GET", "/health", "Health check")
                .tag("ops")
                .response(ResponseDoc { status: 200, description: r#"{"ok":true}"#.into(), schema: None, example: None }),
        )
        .to_openapi_json()
        .to_string()
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    let config = ServerConfig::new().port(3000);
    let bus = LiveBus::new(config.live_bus_capacity);
    let state = AppState {
        csrf: Arc::new(CsrfStore::new()),
        bus: bus.clone(),
        openapi_json: Arc::new(build_openapi()),
    };

    // Heartbeat task
    let hb_bus = bus.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(15));
        loop {
            interval.tick().await;
            hb_bus.heartbeat();
        }
    });

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/submit", post(submit_handler))
        .route("/live/events", get(sse_handler))
        .route("/health", get(health_handler))
        .route("/docs/api/openapi.json", get(openapi_handler))
        .route("/docs", get(docs_handler))
        .fallback(not_found_handler)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let addr = config.addr();
    tracing::info!("Polish listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
