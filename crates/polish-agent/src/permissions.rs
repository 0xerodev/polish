use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PermissionGrant {
    ReadContext,
    WriteOutput,
    CallExternalApi,
    AccessDatabase,
    ModifyState,
    RunSubagent,
    SendNotification,
}

impl PermissionGrant {
    pub fn as_str(&self) -> &str {
        match self {
            PermissionGrant::ReadContext => "read_context",
            PermissionGrant::WriteOutput => "write_output",
            PermissionGrant::CallExternalApi => "call_external_api",
            PermissionGrant::AccessDatabase => "access_database",
            PermissionGrant::ModifyState => "modify_state",
            PermissionGrant::RunSubagent => "run_subagent",
            PermissionGrant::SendNotification => "send_notification",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentPermissions {
    grants: HashSet<PermissionGrant>,
    deny_all: bool,
}

impl AgentPermissions {
    pub fn none() -> Self {
        Self { grants: HashSet::new(), deny_all: false }
    }

    pub fn read_only() -> Self {
        let mut p = Self::none();
        p.grants.insert(PermissionGrant::ReadContext);
        p
    }

    pub fn standard() -> Self {
        let mut p = Self::none();
        p.grants.insert(PermissionGrant::ReadContext);
        p.grants.insert(PermissionGrant::WriteOutput);
        p
    }

    pub fn grant(mut self, permission: PermissionGrant) -> Self {
        self.grants.insert(permission);
        self
    }

    pub fn deny_all(mut self) -> Self {
        self.deny_all = true;
        self
    }

    pub fn check(&self, permission: &PermissionGrant) -> bool {
        if self.deny_all { return false; }
        self.grants.contains(permission)
    }

    pub fn assert_has(&self, permission: &PermissionGrant) -> Result<(), String> {
        if self.check(permission) {
            Ok(())
        } else {
            Err(format!("Agent missing permission: {}", permission.as_str()))
        }
    }

    pub fn list_grants(&self) -> Vec<&PermissionGrant> {
        self.grants.iter().collect()
    }
}
