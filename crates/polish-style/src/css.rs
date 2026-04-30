use crate::theme::Theme;

pub struct ClassName(pub String);

impl ClassName {
    pub fn new(s: impl Into<String>) -> Self { Self(s.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

pub struct CssWriter {
    buf: String,
}

impl CssWriter {
    pub fn new() -> Self { Self { buf: String::with_capacity(8192) } }

    pub fn rule(&mut self, selector: &str, props: &[(&str, &str)]) {
        self.buf.push_str(selector);
        self.buf.push_str(" {\n");
        for (k, v) in props {
            self.buf.push_str("  ");
            self.buf.push_str(k);
            self.buf.push_str(": ");
            self.buf.push_str(v);
            self.buf.push_str(";\n");
        }
        self.buf.push_str("}\n");
    }

    pub fn media(&mut self, query: &str, inner: impl FnOnce(&mut CssWriter)) {
        self.buf.push_str("@media ");
        self.buf.push_str(query);
        self.buf.push_str(" {\n");
        let mut inner_w = CssWriter::new();
        inner(&mut inner_w);
        // indent inner
        for line in inner_w.buf.lines() {
            self.buf.push_str("  ");
            self.buf.push_str(line);
            self.buf.push('\n');
        }
        self.buf.push_str("}\n");
    }

    pub fn raw(&mut self, s: &str) { self.buf.push_str(s); self.buf.push('\n'); }

    pub fn finish(self) -> String { self.buf }
}

impl Default for CssWriter {
    fn default() -> Self { Self::new() }
}

pub struct StyleSheet {
    pub theme_class: String,
    pub css: String,
}

impl StyleSheet {
    /// Generate the full CSS stylesheet for a theme.
    pub fn generate(theme: &Theme) -> Self {
        let mut w = CssWriter::new();
        let c = &theme.colors;
        let sp = &theme.spacing;
        let r = &theme.radius;
        let t = &theme.typography;
        let m = &theme.motion;
        let sh = &theme.shadow;
        let bp = &theme.breakpoints;

        // CSS custom properties
        w.rule(&format!(".{}", theme.body_class), &[
            ("--p-bg",          c.bg_primary),
            ("--p-bg2",         c.bg_secondary),
            ("--p-bg-card",     c.bg_card),
            ("--p-border",      c.border),
            ("--p-border-sub",  c.border_subtle),
            ("--p-text",        c.text_primary),
            ("--p-text2",       c.text_secondary),
            ("--p-muted",       c.text_muted),
            ("--p-accent",      c.accent),
            ("--p-accent-h",    c.accent_hover),
            ("--p-ok",          c.success),
            ("--p-warn",        c.warning),
            ("--p-err",         c.error),
            ("--p-link",        c.link),
            ("--p-sp-xs",       sp.xs),
            ("--p-sp-sm",       sp.sm),
            ("--p-sp-md",       sp.md),
            ("--p-sp-lg",       sp.lg),
            ("--p-sp-xl",       sp.xl),
            ("--p-sp-xxl",      sp.xxl),
            ("--p-r-sm",        r.sm),
            ("--p-r-md",        r.md),
            ("--p-r-lg",        r.lg),
            ("--p-font",        t.font_family),
            ("--p-font-mono",   t.font_mono),
            ("--p-text-base",   t.size_base),
            ("--p-lh",          t.line_height),
            ("--p-dur-fast",    m.fast),
            ("--p-dur-base",    m.base),
            ("--p-easing",      m.easing),
            ("--p-sh-sm",       sh.sm),
            ("--p-sh-md",       sh.md),
            ("--p-sh-lg",       sh.lg),
            ("--p-sh-glass",    sh.glass),
            ("background",      "var(--p-bg)"),
            ("color",           "var(--p-text)"),
            ("font-family",     "var(--p-font)"),
            ("font-size",       "var(--p-text-base)"),
            ("line-height",     "var(--p-lh)"),
        ]);

        // Reset
        w.raw("*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }");

        // App shell
        w.rule(".p-app-shell", &[
            ("min-height", "100vh"),
            ("display", "flex"),
            ("flex-direction", "column"),
        ]);

        w.rule(".p-main", &[
            ("flex", "1"),
            ("padding", "var(--p-sp-lg)"),
            ("max-width", "1200px"),
            ("margin", "0 auto"),
            ("width", "100%"),
        ]);

        // Header
        w.rule(".p-header", &[
            ("background", "var(--p-bg2)"),
            ("border-bottom", "1px solid var(--p-border)"),
            ("padding", "var(--p-sp-md) var(--p-sp-lg)"),
            ("display", "flex"),
            ("align-items", "center"),
            ("gap", "var(--p-sp-md)"),
        ]);

        w.rule(".p-header-title", &[
            ("font-size", t.size_lg),
            ("font-weight", t.weight_semibold),
            ("color", "var(--p-text)"),
        ]);

        // Card / Glass card
        w.rule(".p-card", &[
            ("background", "var(--p-bg-card)"),
            ("border", "1px solid var(--p-border)"),
            ("border-radius", "var(--p-r-lg)"),
            ("padding", "var(--p-sp-lg)"),
            ("box-shadow", "var(--p-sh-md)"),
        ]);

        w.rule(".p-glass-card", &[
            ("background", "var(--p-bg-card)"),
            ("border", "1px solid var(--p-border)"),
            ("border-radius", "var(--p-r-lg)"),
            ("padding", "var(--p-sp-lg)"),
            ("box-shadow", "var(--p-sh-glass)"),
            ("backdrop-filter", "blur(12px)"),
            ("-webkit-backdrop-filter", "blur(12px)"),
        ]);

        // Form elements
        w.rule(".p-form", &[
            ("display", "flex"),
            ("flex-direction", "column"),
            ("gap", "var(--p-sp-md)"),
        ]);

        w.rule(".p-field", &[
            ("display", "flex"),
            ("flex-direction", "column"),
            ("gap", "var(--p-sp-xs)"),
        ]);

        w.rule(".p-label", &[
            ("font-size", t.size_sm),
            ("font-weight", t.weight_medium),
            ("color", "var(--p-text2)"),
        ]);

        w.rule(".p-input", &[
            ("background", "var(--p-bg2)"),
            ("border", "1px solid var(--p-border)"),
            ("border-radius", "var(--p-r-md)"),
            ("padding", &format!("{} {}", sp.sm, sp.md)),
            ("color", "var(--p-text)"),
            ("font-family", "var(--p-font)"),
            ("font-size", "var(--p-text-base)"),
            ("width", "100%"),
            ("transition", "border-color var(--p-dur-fast) var(--p-easing)"),
            ("outline", "none"),
        ]);

        w.rule(".p-input:focus", &[
            ("border-color", "var(--p-accent)"),
            ("box-shadow", "0 0 0 3px rgba(34,167,224,0.15)"),
        ]);

        w.rule(".p-input.p-error", &[
            ("border-color", "var(--p-err)"),
        ]);

        w.rule(".p-field-error", &[
            ("color", "var(--p-err)"),
            ("font-size", t.size_sm),
        ]);

        // Buttons
        w.rule(".p-btn", &[
            ("display", "inline-flex"),
            ("align-items", "center"),
            ("justify-content", "center"),
            ("gap", "var(--p-sp-xs)"),
            ("padding", &format!("{} {}", sp.sm, sp.lg)),
            ("border-radius", "var(--p-r-md)"),
            ("font-family", "var(--p-font)"),
            ("font-size", "var(--p-text-base)"),
            ("font-weight", t.weight_medium),
            ("cursor", "pointer"),
            ("border", "none"),
            ("text-decoration", "none"),
            ("transition", "all var(--p-dur-fast) var(--p-easing)"),
            ("user-select", "none"),
        ]);

        w.rule(".p-btn-primary", &[
            ("background", "var(--p-accent)"),
            ("color", "#fff"),
        ]);

        w.rule(".p-btn-primary:hover:not(:disabled)", &[
            ("background", "var(--p-accent-h)"),
            ("transform", "translateY(-1px)"),
        ]);

        w.rule(".p-btn-secondary", &[
            ("background", "var(--p-bg2)"),
            ("color", "var(--p-text)"),
            ("border", "1px solid var(--p-border)"),
        ]);

        w.rule(".p-btn-secondary:hover:not(:disabled)", &[
            ("background", "var(--p-bg)"),
            ("border-color", "var(--p-accent)"),
        ]);

        w.rule(".p-btn-danger", &[
            ("background", "var(--p-err)"),
            ("color", "#fff"),
        ]);

        w.rule(".p-btn:disabled", &[
            ("opacity", "0.5"),
            ("cursor", "not-allowed"),
        ]);

        // Result strips
        w.rule(".p-result-strip", &[
            ("padding", &format!("{} {}", sp.md, sp.lg)),
            ("border-radius", "var(--p-r-md)"),
            ("border-left", "3px solid transparent"),
        ]);

        w.rule(".p-result-strip.p-ok", &[
            ("background", "rgba(34,197,94,0.08)"),
            ("border-left-color", "var(--p-ok)"),
            ("color", "var(--p-ok)"),
        ]);

        w.rule(".p-result-strip.p-err", &[
            ("background", "rgba(239,68,68,0.08)"),
            ("border-left-color", "var(--p-err)"),
            ("color", "var(--p-err)"),
        ]);

        // Status pills
        w.rule(".p-pill", &[
            ("display", "inline-flex"),
            ("align-items", "center"),
            ("padding", &format!("2px {}", sp.sm)),
            ("border-radius", "var(--p-r-full, 9999px)"),
            ("font-size", t.size_xs),
            ("font-weight", t.weight_medium),
        ]);

        w.rule(".p-pill-ok",   &[("background","rgba(34,197,94,0.12)"),("color","var(--p-ok)")]);
        w.rule(".p-pill-warn", &[("background","rgba(245,158,11,0.12)"),("color","var(--p-warn)")]);
        w.rule(".p-pill-err",  &[("background","rgba(239,68,68,0.12)"),("color","var(--p-err)")]);
        w.rule(".p-pill-neutral",&[("background","rgba(140,160,176,0.12)"),("color","var(--p-text2)")]);

        // Table
        w.rule(".p-table", &[
            ("width", "100%"),
            ("border-collapse", "collapse"),
        ]);

        w.rule(".p-table th, .p-table td", &[
            ("padding", &format!("{} {}", sp.sm, sp.md)),
            ("text-align", "left"),
            ("border-bottom", "1px solid var(--p-border-sub)"),
        ]);

        w.rule(".p-table th", &[
            ("font-size", t.size_sm),
            ("font-weight", t.weight_semibold),
            ("color", "var(--p-text2)"),
            ("background", "var(--p-bg2)"),
        ]);

        // Metric block
        w.rule(".p-metric", &[
            ("display", "flex"),
            ("flex-direction", "column"),
            ("gap", "var(--p-sp-xs)"),
        ]);

        w.rule(".p-metric-label", &[
            ("font-size", t.size_xs),
            ("color", "var(--p-muted)"),
            ("text-transform", "uppercase"),
            ("letter-spacing", "0.06em"),
        ]);

        w.rule(".p-metric-value", &[
            ("font-size", t.size_2xl),
            ("font-weight", t.weight_semibold),
            ("font-variant-numeric", "tabular-nums"),
            ("letter-spacing", "-0.01em"),
        ]);

        // Stack/Inline layout helpers
        w.rule(".p-stack",  &[("display","flex"),("flex-direction","column"),("gap","var(--p-sp-md)")]);
        w.rule(".p-inline", &[("display","flex"),("align-items","center"),("gap","var(--p-sp-md)")]);
        w.rule(".p-grid-2", &[("display","grid"),("grid-template-columns","repeat(2,1fr)"),("gap","var(--p-sp-md)")]);
        w.rule(".p-grid-3", &[("display","grid"),("grid-template-columns","repeat(3,1fr)"),("gap","var(--p-sp-md)")]);

        // Code block
        w.rule(".p-code", &[
            ("font-family", "var(--p-font-mono)"),
            ("font-size", t.size_sm),
            ("background", "var(--p-bg2)"),
            ("border", "1px solid var(--p-border)"),
            ("border-radius", "var(--p-r-md)"),
            ("padding", "var(--p-sp-md)"),
            ("overflow-x", "auto"),
            ("white-space", "pre"),
        ]);

        // Responsive
        w.media(&format!("(max-width: {}px)", bp.md), |mw| {
            mw.rule(".p-main", &[("padding", sp.md)]);
            mw.rule(".p-grid-2, .p-grid-3", &[("grid-template-columns","1fr")]);
            mw.rule(".p-header", &[("padding", &format!("{} {}", sp.sm, sp.md))]);
        });

        // Monospace / proof elements
        w.rule(".p-mono", &[
            ("font-family", "var(--p-font-mono)"),
            ("font-size", t.size_sm),
            ("color", "var(--p-text2)"),
        ]);

        // Loading / skeleton
        w.rule(".p-loading", &[
            ("display", "inline-block"),
            ("width", "20px"),
            ("height", "20px"),
            ("border", "2px solid var(--p-border)"),
            ("border-top-color", "var(--p-accent)"),
            ("border-radius", "50%"),
            ("animation", "p-spin 0.8s linear infinite"),
        ]);

        w.raw("@keyframes p-spin { to { transform: rotate(360deg); } }");

        // Section / panel
        w.rule(".p-section", &[
            ("margin-bottom", "var(--p-sp-xl)"),
        ]);

        w.rule(".p-section-title", &[
            ("font-size", t.size_lg),
            ("font-weight", t.weight_semibold),
            ("margin-bottom", "var(--p-sp-md)"),
            ("padding-bottom", "var(--p-sp-sm)"),
            ("border-bottom", "1px solid var(--p-border-sub)"),
        ]);

        // Command bar
        w.rule(".p-command-bar", &[
            ("display", "flex"),
            ("align-items", "center"),
            ("gap", "var(--p-sp-sm)"),
            ("padding", "var(--p-sp-sm) var(--p-sp-md)"),
            ("background", "var(--p-bg2)"),
            ("border-bottom", "1px solid var(--p-border)"),
        ]);

        // Footer
        w.rule(".p-footer", &[
            ("padding", "var(--p-sp-lg)"),
            ("border-top", "1px solid var(--p-border)"),
            ("color", "var(--p-muted)"),
            ("font-size", t.size_sm),
            ("text-align", "center"),
        ]);

        // Docs specific
        w.rule(".p-docs-sidebar", &[
            ("width", "240px"),
            ("min-width", "240px"),
            ("border-right", "1px solid var(--p-border)"),
            ("padding", "var(--p-sp-lg)"),
            ("overflow-y", "auto"),
        ]);

        w.rule(".p-docs-content", &[
            ("flex", "1"),
            ("padding", "var(--p-sp-xl)"),
            ("max-width", "800px"),
            ("overflow-y", "auto"),
        ]);

        w.rule(".p-docs-layout", &[
            ("display", "flex"),
            ("height", "100vh"),
        ]);

        // Endpoint block (docs)
        w.rule(".p-endpoint-block", &[
            ("border", "1px solid var(--p-border)"),
            ("border-radius", "var(--p-r-md)"),
            ("overflow", "hidden"),
            ("margin-bottom", "var(--p-sp-lg)"),
        ]);

        w.rule(".p-endpoint-header", &[
            ("display", "flex"),
            ("align-items", "center"),
            ("gap", "var(--p-sp-sm)"),
            ("padding", "var(--p-sp-sm) var(--p-sp-md)"),
            ("background", "var(--p-bg2)"),
            ("border-bottom", "1px solid var(--p-border)"),
        ]);

        w.rule(".p-method-pill", &[
            ("font-family", "var(--p-font-mono)"),
            ("font-size", t.size_xs),
            ("font-weight", t.weight_bold),
            ("padding", &format!("2px {}", sp.sm)),
            ("border-radius", r.sm),
        ]);

        w.rule(".p-method-get",    &[("background","rgba(34,197,94,0.15)"),("color","var(--p-ok)")]);
        w.rule(".p-method-post",   &[("background","rgba(34,167,224,0.15)"),("color","var(--p-accent)")]);
        w.rule(".p-method-put",    &[("background","rgba(245,158,11,0.15)"),("color","var(--p-warn)")]);
        w.rule(".p-method-delete", &[("background","rgba(239,68,68,0.15)"),("color","var(--p-err)")]);

        // SSE / live stream indicator
        w.rule(".p-live-indicator", &[
            ("display", "inline-flex"),
            ("align-items", "center"),
            ("gap", "var(--p-sp-xs)"),
            ("font-size", t.size_xs),
            ("color", "var(--p-ok)"),
        ]);

        w.rule(".p-live-dot", &[
            ("width", "6px"),
            ("height", "6px"),
            ("border-radius", "50%"),
            ("background", "var(--p-ok)"),
            ("animation", "p-pulse 2s ease-in-out infinite"),
        ]);

        w.raw("@keyframes p-pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.3; } }");

        StyleSheet {
            theme_class: theme.body_class.to_string(),
            css: w.finish(),
        }
    }
}
