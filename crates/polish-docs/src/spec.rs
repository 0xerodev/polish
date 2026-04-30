use serde::{Serialize, Deserialize};
use indexmap::IndexMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParamDoc {
    pub name: String,
    pub location: String, // "body", "query", "path", "header"
    pub description: String,
    pub required: bool,
    pub schema_type: String,
    pub example: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseDoc {
    pub status: u16,
    pub description: String,
    pub schema: Option<serde_json::Value>,
    pub example: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndpointDoc {
    pub method: String,
    pub path: String,
    pub summary: String,
    pub description: String,
    pub tags: Vec<String>,
    pub auth_required: bool,
    pub params: Vec<ParamDoc>,
    pub responses: Vec<ResponseDoc>,
    pub example_request: Option<serde_json::Value>,
    pub deprecated: bool,
}

impl EndpointDoc {
    pub fn new(method: impl Into<String>, path: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            method: method.into(),
            path: path.into(),
            summary: summary.into(),
            description: String::new(),
            tags: Vec::new(),
            auth_required: true,
            params: Vec::new(),
            responses: Vec::new(),
            example_request: None,
            deprecated: false,
        }
    }

    pub fn description(mut self, d: impl Into<String>) -> Self { self.description = d.into(); self }
    pub fn tag(mut self, t: impl Into<String>) -> Self { self.tags.push(t.into()); self }
    pub fn no_auth(mut self) -> Self { self.auth_required = false; self }
    pub fn param(mut self, p: ParamDoc) -> Self { self.params.push(p); self }
    pub fn response(mut self, r: ResponseDoc) -> Self { self.responses.push(r); self }
    pub fn deprecated(mut self) -> Self { self.deprecated = true; self }

    pub fn method_lower(&self) -> &str { &self.method }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SchemaDoc {
    pub name: String,
    pub description: String,
    pub fields: Vec<SchemaField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub description: String,
    pub example: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub title: String,
    pub version: String,
    pub description: String,
    pub servers: Vec<String>,
    pub endpoints: Vec<EndpointDoc>,
    pub schemas: Vec<SchemaDoc>,
}

impl OpenApiSpec {
    pub fn new(title: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            version: version.into(),
            description: String::new(),
            servers: Vec::new(),
            endpoints: Vec::new(),
            schemas: Vec::new(),
        }
    }

    pub fn description(mut self, d: impl Into<String>) -> Self { self.description = d.into(); self }
    pub fn server(mut self, s: impl Into<String>) -> Self { self.servers.push(s.into()); self }
    pub fn endpoint(mut self, e: EndpointDoc) -> Self { self.endpoints.push(e); self }
    pub fn schema(mut self, s: SchemaDoc) -> Self { self.schemas.push(s); self }

    pub fn to_openapi_json(&self) -> serde_json::Value {
        let mut paths: IndexMap<String, serde_json::Value> = IndexMap::new();
        for ep in &self.endpoints {
            let method = ep.method.to_lowercase();
            let mut op = serde_json::json!({
                "summary": ep.summary,
                "description": ep.description,
                "tags": ep.tags,
                "deprecated": ep.deprecated,
                "responses": {}
            });
            if ep.auth_required {
                op["security"] = serde_json::json!([{"bearerAuth": []}]);
            }
            let mut responses = serde_json::Map::new();
            for r in &ep.responses {
                responses.insert(r.status.to_string(), serde_json::json!({
                    "description": r.description
                }));
            }
            op["responses"] = serde_json::Value::Object(responses);
            paths.entry(ep.path.clone()).or_insert_with(|| serde_json::json!({}))
                .as_object_mut().unwrap().insert(method, op);
        }

        serde_json::json!({
            "openapi": "3.1.0",
            "info": {
                "title": self.title,
                "version": self.version,
                "description": self.description
            },
            "servers": self.servers.iter().map(|s| serde_json::json!({"url": s})).collect::<Vec<_>>(),
            "paths": paths,
            "components": {
                "securitySchemes": {
                    "bearerAuth": {
                        "type": "http",
                        "scheme": "bearer"
                    }
                }
            }
        })
    }
}
