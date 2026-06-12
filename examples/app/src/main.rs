//! Polish example server — real axum HTTP, SSE live updates, CSRF form processing.

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

static ORDER_COUNT: AtomicU64 = AtomicU64::new(0);

use axum::{
    Router,
    extract::{Form, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
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

#[derive(Clone)]
struct AppState {
    csrf: Arc<CsrfStore>,
    bus: LiveBus,
    openapi_json: Arc<String>,
}

const CSP: &str = "default-src 'self'; img-src 'self' data:; script-src 'unsafe-inline'; style-src 'unsafe-inline'";

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

const LIVE_JS: &str = r#"<script>
(function(){
  var es = new EventSource('/live/events');
  es.addEventListener('fragment', function(e){
    var f = JSON.parse(e.data);
    var el = document.getElementById(f.target);
    if (!el) return;
    var tmpl = document.createElement('template');
    tmpl.innerHTML = f.html;
    if (f.op === 'replace') el.replaceWith(tmpl.content.firstChild || el);
    else if (f.op === 'append')  el.appendChild(tmpl.content.firstChild);
    else if (f.op === 'prepend') el.prepend(tmpl.content.firstChild);
    else if (f.op === 'remove')  el.remove();
  });
  es.addEventListener('heartbeat', function(){});
  es.onerror = function(){ setTimeout(function(){ location.reload(); }, 3000); };
})();
</script>"#;

const SUBMIT_JS: &str = r#"<script>
(function(){
  var form = document.querySelector('form');
  var btn  = form && form.querySelector('button[type="submit"]');
  if (!form || !btn) return;
  form.addEventListener('submit', function(){
    btn.disabled = true;
    btn.textContent = 'Submitting…';
  });
})();
</script>"#;

fn nav(active: &str) -> String {
    let link = |href: &str, label: &str| {
        let active_style = if href == active {
            " style=\"color:var(--p-accent);border-bottom:2px solid var(--p-accent);padding-bottom:2px\""
        } else {
            ""
        };
        format!(r#"<a href="{href}" class="p-nav-link"{active_style}>{label}</a>"#)
    };
    format!(
        r#"<nav class="p-header" style="display:flex;align-items:center;gap:2rem;padding:1rem 2rem;border-bottom:1px solid var(--p-border);position:sticky;top:0;z-index:10;backdrop-filter:blur(8px)">
  <span class="p-header-title" style="font-size:1.1rem;font-weight:700;letter-spacing:-0.01em">Polish</span>
  <div style="display:flex;gap:1.5rem">
    {order}{docs}
  </div>
  <div style="margin-left:auto;display:flex;gap:1rem;font-size:.75rem;color:var(--p-text2)"><span id="live-orders" title="updates live over SSE">orders: {n}</span><span>v0.1.0</span></div>
</nav>"#,
        n = ORDER_COUNT.load(Ordering::Relaxed),
        order = link("/", "Order Form"),
        docs = link("/docs", "Docs"),
    )
}

fn themed_page(title: &str, body_html: &str, include_live: bool, active_nav: &str) -> String {
    let theme = BuiltinTheme::GlassHud.theme();
    let css = StyleSheet::generate(&theme);
    let live_script = if include_live { LIVE_JS } else { "" };
    let submit_script = if include_live { SUBMIT_JS } else { "" };
    format!(
        r#"<!DOCTYPE html>
<html lang="en" class="{tc}">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <link rel="icon" href="data:image/svg+xml,%3Csvg xmlns=%27http://www.w3.org/2000/svg%27 viewBox=%270 0 16 16%27%3E%3Crect width=%2716%27 height=%2716%27 rx=%273%27 fill=%27%232563eb%27/%3E%3Cpath d=%27M9 2 4 9h3l-1 5 5-7H8z%27 fill=%27white%27/%3E%3C/svg%3E">
  <title>{title}</title>
  <style>
{css}
html,body{{min-height:100%;margin:0}}
body{{display:flex;flex-direction:column;min-height:100vh}}
.p-page-body{{flex:1;display:flex;flex-direction:column}}
  </style>
</head>
<body>
{nav}
<div class="p-page-body">
{body}
</div>
{live}{submit}
</body>
</html>"#,
        tc = css.theme_class,
        css = css.css,
        title = escape_html(title),
        nav = nav(active_nav),
        body = body_html,
        live = live_script,
        submit = submit_script,
    )
}

fn centered_card(content: &str) -> String {
    format!(
        r#"<div style="flex:1;display:flex;align-items:center;justify-content:center;padding:2rem">
  <div class="p-card" style="width:100%;max-width:480px">{content}</div>
</div>"#
    )
}

// ── Routes ────────────────────────────────────────────────────────────────────

async fn index_handler(State(state): State<AppState>) -> Response {
    let token = state.csrf.issue("anon");
    let form_def = make_form();
    let ctx = RenderContext::default();
    use polish_core::Render;
    let form_html = FormComponent {
        form: &form_def,
        csrf_token: Some(&token),
        submit_label: "Place Order",
    }.to_html(&ctx);

    let content = format!(
        r#"<h2 style="margin:0 0 0.25rem">Order Form</h2>
<p style="margin:0 0 1.5rem;color:var(--p-text2);font-size:.875rem">Fill in your details to place an order.</p>
<div id="result"></div>
{form}"#,
        form = form_html
    );
    secure_html(StatusCode::OK, themed_page("Order Form — Polish", &centered_card(&content), true, "/"))
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
                let n = ORDER_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                bus.fragment(LiveFragment::replace("live-orders", format!("orders: {n}")));
            }
            let html = match r.outcome {
                ActionOutput::Replace { page_html } => page_html,
                _ => String::new(),
            };
            let content = format!(
                r#"<div style="text-align:center;padding:0.5rem 0 1rem">
  <div style="font-size:3rem;margin-bottom:1rem">✓</div>
  <h2 style="margin:0 0 0.5rem">Order Placed!</h2>
</div>
{html}
<div style="margin-top:1.5rem">
  <a href="/" class="p-btn p-btn-secondary" style="width:100%;text-align:center">Place Another Order</a>
</div>"#
            );
            secure_html(StatusCode::OK, themed_page("Order Placed — Polish", &centered_card(&content), false, "/"))
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

            // Distinguish CSRF failures from field validation errors
            let error_msg = match e {
                polish_actions::ActionError::InvalidCsrf =>
                    "Session expired. Please try again.".to_string(),
                _ => "Please fix the errors below.".to_string(),
            };

            let content = format!(
                r#"<h2 style="margin:0 0 0.25rem">Order Form</h2>
<p style="margin:0 0 1.5rem;color:var(--p-text2);font-size:.875rem">Fill in your details to place an order.</p>
<div class="p-result-strip p-err" style="margin-bottom:1rem">{err}</div>
<div id="result"></div>
{form}"#,
                err = escape_html(&error_msg),
                form = form_html
            );
            secure_html(StatusCode::BAD_REQUEST, themed_page("Error — Polish", &centered_card(&content), true, "/"))
        }
    }
}

async fn sse_handler(State(state): State<AppState>) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.bus.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| async move {
        let event = msg.ok()?;
        let mut e = Event::default().data(event.data);
        if event.kind != EventKind::Message {
            e = e.event(event.kind.as_str());
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
                .content("execute_pipeline(): CSRF check → validate → handler → ActionResult. One call does it all."))
            .section(DocsSection::new("Security", "security")
                .content("All responses include CSP, X-Frame-Options: DENY, X-Content-Type-Options: nosniff, Referrer-Policy. User input is HTML-escaped before rendering.")));

    let mut body = r#"<div style="max-width:780px;margin:0 auto;padding:2rem">"#.to_owned();
    body.push_str(&format!("<h1 style=\"margin-bottom:0.25rem\">{}</h1>", escape_html(&docs_site.title)));
    body.push_str(r#"<p style="color:var(--p-text2);margin-bottom:2rem">Rust-first server-authoritative frontend platform</p>"#);
    for page in &docs_site.pages {
        body.push_str(&format!("<h2 style=\"margin:2rem 0 1rem\">{}</h2>", escape_html(&page.title)));
        for section in &page.sections {
            body.push_str(&format!(
                r#"<div class="p-card" style="margin-bottom:1rem"><h3 style="margin:0 0 0.5rem">{}</h3><p style="margin:0;color:var(--p-text2)">{}</p></div>"#,
                escape_html(&section.title), escape_html(&section.content)
            ));
        }
    }
    body.push_str("</div>");
    secure_html(StatusCode::OK, themed_page("Polish Docs", &body, false, "/docs"))
}

async fn not_found_handler() -> Response {
    let content = r#"<div style="text-align:center;padding:1rem 0">
  <div style="font-size:4rem;font-weight:800;color:var(--p-accent);margin-bottom:0.5rem">404</div>
  <h2 style="margin:0 0 0.5rem">Page not found</h2>
  <p style="color:var(--p-text2);margin:0 0 1.5rem">The page you're looking for doesn't exist.</p>
  <a href="/" class="p-btn p-btn-primary">Go Home</a>
</div>"#;
    secure_html(StatusCode::NOT_FOUND, themed_page("Not Found — Polish", &centered_card(content), false, ""))
}

// ── Factories ─────────────────────────────────────────────────────────────────

fn make_form() -> PolishForm {
    PolishForm::new("order-form", "/submit")
        .field(FieldMeta::new("name", "Full Name").required().placeholder("Alice Smith"))
        .field(FieldMeta::new("email", "Email").required().email().placeholder("alice@example.com"))
        .field(FieldMeta::new("amount", "Amount (USD)").required().number().placeholder("100"))
}

fn make_validator() -> Validator {
    Validator::new()
        .label("name", "Full Name")
        .label("email", "Email")
        .label("amount", "Amount")
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
            EndpointDoc::new("GET", "/live/events", "SSE live event stream")
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

    let hb_bus = bus.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(15));
        loop { interval.tick().await; hb_bus.heartbeat(); }
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
