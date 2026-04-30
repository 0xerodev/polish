use crate::tokens::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ThemeMode { Light, Dark }

pub struct Theme {
    pub name: &'static str,
    pub mode: ThemeMode,
    pub colors: ColorPalette,
    pub spacing: Spacing,
    pub radius: Radius,
    pub typography: Typography,
    pub motion: Motion,
    pub breakpoints: Breakpoints,
    pub shadow: Shadow,
    pub body_class: &'static str,
}

pub enum BuiltinTheme { GlassHud, Clean, Enterprise }

impl BuiltinTheme {
    pub fn theme(&self) -> Theme {
        match self {
            BuiltinTheme::GlassHud => glass_hud_theme(),
            BuiltinTheme::Clean => clean_theme(),
            BuiltinTheme::Enterprise => enterprise_theme(),
        }
    }
}

fn glass_hud_theme() -> Theme {
    Theme {
        name: "GlassHud",
        mode: ThemeMode::Dark,
        body_class: "p-theme-glass",
        colors: ColorPalette {
            bg_primary:    "#0a0f14",
            bg_secondary:  "#0f1620",
            bg_card:       "rgba(15,22,32,0.72)",
            bg_overlay:    "rgba(10,15,20,0.88)",
            border:        "rgba(34,50,65,0.9)",
            border_subtle: "rgba(34,50,65,0.4)",
            text_primary:  "#e8edf2",
            text_secondary:"#8ca0b0",
            text_muted:    "#4a6070",
            accent:        "#22a7e0",
            accent_hover:  "#1d90c0",
            success:       "#22c55e",
            warning:       "#f59e0b",
            error:         "#ef4444",
            link:          "#22a7e0",
        },
        spacing: Spacing {
            xs: "4px", sm: "8px", md: "16px",
            lg: "24px", xl: "32px", xxl: "48px",
        },
        radius: Radius {
            sm: "4px", md: "8px", lg: "12px", full: "9999px",
        },
        typography: Typography {
            font_family: "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
            font_mono:   "'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace",
            size_xs:   "11px", size_sm:   "13px", size_base: "15px",
            size_lg:   "17px", size_xl:   "20px", size_2xl:  "24px", size_3xl: "32px",
            weight_normal:   "400", weight_medium:   "500",
            weight_semibold: "600", weight_bold:     "700",
            line_height: "1.6",
        },
        motion: Motion {
            fast: "120ms", base: "200ms", slow: "400ms",
            easing: "cubic-bezier(0.4, 0, 0.2, 1)",
        },
        breakpoints: Breakpoints { sm: 640, md: 768, lg: 1024, xl: 1280 },
        shadow: Shadow {
            sm:    "0 1px 3px rgba(0,0,0,0.4)",
            md:    "0 4px 12px rgba(0,0,0,0.5)",
            lg:    "0 8px 32px rgba(0,0,0,0.6)",
            glass: "0 4px 24px rgba(0,0,0,0.5), inset 0 1px 0 rgba(255,255,255,0.06)",
        },
    }
}

fn clean_theme() -> Theme {
    Theme {
        name: "Clean",
        mode: ThemeMode::Light,
        body_class: "p-theme-clean",
        colors: ColorPalette {
            bg_primary:    "#ffffff",
            bg_secondary:  "#f8fafc",
            bg_card:       "#ffffff",
            bg_overlay:    "rgba(255,255,255,0.96)",
            border:        "#e2e8f0",
            border_subtle: "#f1f5f9",
            text_primary:  "#0f172a",
            text_secondary:"#475569",
            text_muted:    "#94a3b8",
            accent:        "#2563eb",
            accent_hover:  "#1d4ed8",
            success:       "#16a34a",
            warning:       "#d97706",
            error:         "#dc2626",
            link:          "#2563eb",
        },
        spacing: Spacing {
            xs: "4px", sm: "8px", md: "16px",
            lg: "24px", xl: "32px", xxl: "48px",
        },
        radius: Radius { sm: "4px", md: "6px", lg: "10px", full: "9999px" },
        typography: Typography {
            font_family: "'Inter', system-ui, sans-serif",
            font_mono:   "'JetBrains Mono', monospace",
            size_xs: "11px", size_sm: "13px", size_base: "15px",
            size_lg: "17px", size_xl: "20px", size_2xl: "24px", size_3xl: "32px",
            weight_normal: "400", weight_medium: "500",
            weight_semibold: "600", weight_bold: "700",
            line_height: "1.6",
        },
        motion: Motion {
            fast: "100ms", base: "160ms", slow: "320ms",
            easing: "cubic-bezier(0.4, 0, 0.2, 1)",
        },
        breakpoints: Breakpoints { sm: 640, md: 768, lg: 1024, xl: 1280 },
        shadow: Shadow {
            sm:    "0 1px 2px rgba(0,0,0,0.05)",
            md:    "0 4px 6px rgba(0,0,0,0.07)",
            lg:    "0 10px 15px rgba(0,0,0,0.1)",
            glass: "0 4px 12px rgba(0,0,0,0.08)",
        },
    }
}

fn enterprise_theme() -> Theme {
    Theme {
        name: "Enterprise",
        mode: ThemeMode::Light,
        body_class: "p-theme-enterprise",
        colors: ColorPalette {
            bg_primary:    "#f9fafb",
            bg_secondary:  "#f3f4f6",
            bg_card:       "#ffffff",
            bg_overlay:    "rgba(249,250,251,0.97)",
            border:        "#d1d5db",
            border_subtle: "#e5e7eb",
            text_primary:  "#111827",
            text_secondary:"#374151",
            text_muted:    "#6b7280",
            accent:        "#4f46e5",
            accent_hover:  "#4338ca",
            success:       "#059669",
            warning:       "#b45309",
            error:         "#b91c1c",
            link:          "#4f46e5",
        },
        spacing: Spacing {
            xs: "4px", sm: "8px", md: "16px",
            lg: "24px", xl: "32px", xxl: "48px",
        },
        radius: Radius { sm: "3px", md: "5px", lg: "8px", full: "9999px" },
        typography: Typography {
            font_family: "'IBM Plex Sans', 'Inter', system-ui, sans-serif",
            font_mono:   "'IBM Plex Mono', monospace",
            size_xs: "11px", size_sm: "13px", size_base: "14px",
            size_lg: "16px", size_xl: "18px", size_2xl: "22px", size_3xl: "28px",
            weight_normal: "400", weight_medium: "500",
            weight_semibold: "600", weight_bold: "700",
            line_height: "1.5",
        },
        motion: Motion {
            fast: "80ms", base: "140ms", slow: "280ms",
            easing: "ease-in-out",
        },
        breakpoints: Breakpoints { sm: 640, md: 768, lg: 1024, xl: 1280 },
        shadow: Shadow {
            sm:    "0 1px 2px rgba(0,0,0,0.06)",
            md:    "0 2px 4px rgba(0,0,0,0.08)",
            lg:    "0 4px 8px rgba(0,0,0,0.1)",
            glass: "0 2px 8px rgba(0,0,0,0.08)",
        },
    }
}
