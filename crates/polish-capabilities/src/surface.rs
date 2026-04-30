use serde::{Serialize, Deserialize};

/// Where a field or action is allowed to appear.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Surface {
    /// Main visible product surface.
    PrimaryUi,
    /// Secondary / expandable surface.
    SecondaryUi,
    /// Collapsed / detail view.
    CollapsedUi,
    /// API response only — never UI.
    ApiOnly,
    /// Documentation only.
    DocsOnly,
    /// Authenticated operators only.
    OperatorOnly,
    /// Permanently hidden — must never appear in any surface.
    HiddenEnforced,
    /// Only callable by agents within permission scope.
    AgentOnly,
    /// Never exposed anywhere — forbidden in all outputs.
    NeverUi,
}

impl Surface {
    pub fn is_user_visible(&self) -> bool {
        matches!(self, Surface::PrimaryUi | Surface::SecondaryUi | Surface::CollapsedUi)
    }

    pub fn is_restricted(&self) -> bool {
        matches!(self, Surface::OperatorOnly | Surface::AgentOnly | Surface::NeverUi | Surface::HiddenEnforced)
    }

    pub fn allows_in_html(&self) -> bool {
        matches!(self, Surface::PrimaryUi | Surface::SecondaryUi | Surface::CollapsedUi)
    }

    pub fn allows_in_api(&self) -> bool {
        !matches!(self, Surface::NeverUi | Surface::HiddenEnforced)
    }

    pub fn allows_in_docs(&self) -> bool {
        !matches!(self, Surface::NeverUi | Surface::HiddenEnforced)
    }
}

impl std::fmt::Display for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Surface::PrimaryUi => "PrimaryUi",
            Surface::SecondaryUi => "SecondaryUi",
            Surface::CollapsedUi => "CollapsedUi",
            Surface::ApiOnly => "ApiOnly",
            Surface::DocsOnly => "DocsOnly",
            Surface::OperatorOnly => "OperatorOnly",
            Surface::HiddenEnforced => "HiddenEnforced",
            Surface::AgentOnly => "AgentOnly",
            Surface::NeverUi => "NeverUi",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SurfacePolicy {
    pub surface: Surface,
    pub label: String,
    pub reason: String,
    pub pii: bool,
    pub audit_on_access: bool,
}

impl SurfacePolicy {
    pub fn new(surface: Surface, label: impl Into<String>, reason: impl Into<String>) -> Self {
        Self { surface, label: label.into(), reason: reason.into(), pii: false, audit_on_access: false }
    }

    pub fn pii(mut self) -> Self { self.pii = true; self }
    pub fn audit(mut self) -> Self { self.audit_on_access = true; self }
}
