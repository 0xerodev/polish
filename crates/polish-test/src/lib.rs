pub mod snapshot;
pub mod route;
pub mod capability;
pub mod action;
pub mod state;
pub mod assert_html;

pub use snapshot::{Snapshot, SnapshotStore};
pub use route::{RouteTestClient, RouteResponse};
pub use capability::{CapabilityTestHarness, LeakageTestResult};
pub use action::{ActionTestHarness, ActionTestResult};
pub use state::{StateMachineHarness};
pub use assert_html::{assert_contains, assert_not_contains, assert_attr, assert_no_script_injection};

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub base_url: String,
    pub snapshot_dir: String,
    pub update_snapshots: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000".into(),
            snapshot_dir: "tests/snapshots".into(),
            update_snapshots: std::env::var("UPDATE_SNAPSHOTS").is_ok(),
        }
    }
}

pub fn test_config() -> TestConfig {
    TestConfig::default()
}
