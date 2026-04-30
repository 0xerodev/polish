/// Design token primitives.

#[derive(Clone, Debug)]
pub struct ColorPalette {
    pub bg_primary: &'static str,
    pub bg_secondary: &'static str,
    pub bg_card: &'static str,
    pub bg_overlay: &'static str,
    pub border: &'static str,
    pub border_subtle: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,
    pub accent: &'static str,
    pub accent_hover: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
    pub link: &'static str,
}

#[derive(Clone, Debug)]
pub struct Spacing {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
    pub xxl: &'static str,
}

#[derive(Clone, Debug)]
pub struct Radius {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub full: &'static str,
}

#[derive(Clone, Debug)]
pub struct Typography {
    pub font_family: &'static str,
    pub font_mono: &'static str,
    pub size_xs: &'static str,
    pub size_sm: &'static str,
    pub size_base: &'static str,
    pub size_lg: &'static str,
    pub size_xl: &'static str,
    pub size_2xl: &'static str,
    pub size_3xl: &'static str,
    pub weight_normal: &'static str,
    pub weight_medium: &'static str,
    pub weight_semibold: &'static str,
    pub weight_bold: &'static str,
    pub line_height: &'static str,
}

#[derive(Clone, Debug)]
pub struct Motion {
    pub fast: &'static str,
    pub base: &'static str,
    pub slow: &'static str,
    pub easing: &'static str,
}

#[derive(Clone, Debug)]
pub struct Breakpoints {
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
}

#[derive(Clone, Debug)]
pub struct Shadow {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub glass: &'static str,
}
