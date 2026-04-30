# Polish

Rust-first, server-authoritative frontend framework. The server renders HTML; the browser just displays it. No JavaScript framework required.

## What it is

Polish is a full-stack Rust framework where every HTTP response is HTML rendered on the server. Live updates arrive as DOM fragments over SSE. Forms have built-in CSRF protection, validation with human-readable labels, and XSS-safe output. The browser never holds application state.

## Quickstart

```bash
# Install the CLI
cargo install --path crates/polish-cli

# Create a new app
polish new my-app
cd my-app && polish dev
```

The dev server starts on `http://localhost:3000`.

## Crates

| Crate | Purpose |
|---|---|
| `polish-core` | HTML writer, component model, XSS-safe escaping |
| `polish-style` | GlassHud design system, CSS generation, theme tokens |
| `polish-actions` | Form parsing, validation, CSRF tokens |
| `polish-state` | Server-side state management |
| `polish-server` | axum integration, LiveBus SSE, ServerConfig |
| `polish-live` | SSE event types, live fragment patching |
| `polish-docs` | Documentation site generator |
| `polish-capabilities` | Capability registry and leakage detection |
| `polish-test` | Snapshot and capability testing utilities |
| `polish-visual` | Visual diff and screenshot comparison |
| `polish-agent` | Agent task runner integration |
| `polish-cli` | `polish` CLI — new, dev, build, test, deploy |

## Running the example

```bash
cargo run -p polish-example-app
# or
polish dev
```

Serves on `http://localhost:3000` with:
- Order form with validation and CSRF
- Live SSE event stream at `/live/events`
- OpenAPI docs at `/docs`
- Health check at `/health`

## Design principles

- Server renders all HTML. Browser displays it.
- Live updates via SSE fragment patches — no full-page reload.
- CSRF built-in and single-use. Every form is protected.
- All user input is XSS-escaped before HTML interpolation.
- Zero JavaScript framework dependency.
- `cargo clippy --all-targets -- -D warnings` must pass at all times.

## License

MIT
