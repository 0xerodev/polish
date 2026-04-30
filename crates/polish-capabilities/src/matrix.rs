
use serde::{Serialize, Deserialize};
use crate::surface::{Surface, SurfacePolicy};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityEntry {
    pub id: String,
    pub source: String,
    pub policy: SurfacePolicy,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CapabilityRow {
    pub component: String,
    pub entries: Vec<CapabilityEntry>,
}

impl CapabilityRow {
    pub fn new(component: impl Into<String>) -> Self {
        Self { component: component.into(), entries: Vec::new() }
    }

    pub fn add(mut self, id: impl Into<String>, source: impl Into<String>, policy: SurfacePolicy) -> Self {
        self.entries.push(CapabilityEntry { id: id.into(), source: source.into(), policy });
        self
    }

    pub fn surface_of(&self, id: &str) -> Option<&Surface> {
        self.entries.iter().find(|e| e.id == id).map(|e| &e.policy.surface)
    }

    pub fn visible_entries(&self) -> Vec<&CapabilityEntry> {
        self.entries.iter().filter(|e| e.policy.surface.is_user_visible()).collect()
    }

    pub fn forbidden_entries(&self) -> Vec<&CapabilityEntry> {
        self.entries.iter().filter(|e| matches!(e.policy.surface, Surface::NeverUi | Surface::HiddenEnforced)).collect()
    }
}

/// Full capability matrix for the application.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CapabilityMatrix {
    pub rows: Vec<CapabilityRow>,
}

impl CapabilityMatrix {
    pub fn new() -> Self { Self::default() }

    pub fn add_row(mut self, row: CapabilityRow) -> Self {
        self.rows.push(row); self
    }

    pub fn find_entry(&self, component: &str, id: &str) -> Option<&CapabilityEntry> {
        self.rows.iter()
            .find(|r| r.component == component)
            .and_then(|r| r.entries.iter().find(|e| e.id == id))
    }

    pub fn all_forbidden(&self) -> Vec<(&str, &CapabilityEntry)> {
        self.rows.iter()
            .flat_map(|r| r.forbidden_entries().into_iter().map(|e| (r.component.as_str(), e)))
            .collect()
    }

    pub fn all_pii(&self) -> Vec<(&str, &CapabilityEntry)> {
        self.rows.iter()
            .flat_map(|r| r.entries.iter()
                .filter(|e| e.policy.pii)
                .map(|e| (r.component.as_str(), e)))
            .collect()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Generate a markdown table for docs.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("| Component | Field | Surface | Label | PII |\n");
        out.push_str("|---|---|---|---|---|\n");
        for row in &self.rows {
            for entry in &row.entries {
                out.push_str(&format!(
                    "| {} | {} | {} | {} | {} |\n",
                    row.component, entry.id, entry.policy.surface,
                    entry.policy.label,
                    if entry.policy.pii { "yes" } else { "no" }
                ));
            }
        }
        out
    }
}
