use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ActionTestResult {
    pub ok: bool,
    pub output_kind: String,
    pub message: Option<String>,
    pub redirect_url: Option<String>,
    pub errors: HashMap<String, String>,
}

impl ActionTestResult {
    pub fn assert_ok(&self) -> Result<(), String> {
        if self.ok { Ok(()) } else {
            Err(format!("Action failed: {:?}", self.message))
        }
    }

    pub fn assert_redirect_to(&self, url: &str) -> Result<(), String> {
        match &self.redirect_url {
            Some(u) if u == url => Ok(()),
            Some(u) => Err(format!("Expected redirect to {url:?}, got {u:?}")),
            None => Err("No redirect in action result".into()),
        }
    }

    pub fn assert_field_error(&self, field: &str, contains: &str) -> Result<(), String> {
        match self.errors.get(field) {
            Some(msg) if msg.contains(contains) => Ok(()),
            Some(msg) => Err(format!("Field {field:?} error {msg:?} does not contain {contains:?}")),
            None => Err(format!("No error for field {field:?}")),
        }
    }
}

type ActionHandler = Box<dyn Fn(&HashMap<String, String>) -> ActionTestResult>;

pub struct ActionTestHarness {
    actions: HashMap<String, ActionHandler>,
}

impl ActionTestHarness {
    pub fn new() -> Self {
        Self { actions: HashMap::new() }
    }

    pub fn register(&mut self, name: &str, handler: impl Fn(&HashMap<String, String>) -> ActionTestResult + 'static) {
        self.actions.insert(name.to_string(), Box::new(handler) as ActionHandler);
    }

    pub fn run(&self, name: &str, fields: HashMap<String, String>) -> ActionTestResult {
        if let Some(handler) = self.actions.get(name) {
            handler(&fields)
        } else {
            ActionTestResult {
                ok: false,
                output_kind: "error".into(),
                message: Some(format!("Unknown action: {name}")),
                redirect_url: None,
                errors: HashMap::new(),
            }
        }
    }
}

impl Default for ActionTestHarness {
    fn default() -> Self { Self::new() }
}
