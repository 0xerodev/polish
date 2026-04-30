pub mod html;
pub mod attrs;
pub mod render;
pub mod page;
pub mod context;
pub mod escape;
pub mod error;

pub use html::{HtmlWriter, SafeHtml};
pub use attrs::Attrs;
pub use render::{Render, Component, Fragment, RenderExt};
pub use page::{Page, PageMeta, Slot};
pub use context::RenderContext;
pub use escape::{escape_html, escape_attr, escape_url};
pub use error::CoreError;
