//! A minimal serde model for the subset of OpenAPI 3.0 that the Aevo docs use.
//!
//! This intentionally models only what we consume during code generation:
//! `components.schemas`, `paths` with their operations, parameters, request
//! bodies and responses. Unknown fields are ignored.

use std::collections::BTreeMap;

use serde::Deserialize;

/// Top-level OpenAPI document embedded in each markdown file.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OpenApi {
    #[serde(default)]
    pub components: Components,
    #[serde(default)]
    pub paths: BTreeMap<String, PathItem>,
}

/// `components` object (we only care about schemas).
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Components {
    #[serde(default)]
    pub schemas: BTreeMap<String, Schema>,
}

/// A path item holds one operation per HTTP method.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct PathItem {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub delete: Option<Operation>,
    pub patch: Option<Operation>,
}

impl PathItem {
    /// Iterate over `(method, operation)` for each defined method.
    pub fn operations(&self) -> Vec<(&'static str, &Operation)> {
        let mut out = Vec::new();
        if let Some(op) = &self.get {
            out.push(("GET", op));
        }
        if let Some(op) = &self.post {
            out.push(("POST", op));
        }
        if let Some(op) = &self.put {
            out.push(("PUT", op));
        }
        if let Some(op) = &self.delete {
            out.push(("DELETE", op));
        }
        if let Some(op) = &self.patch {
            out.push(("PATCH", op));
        }
        out
    }
}

/// A single API operation.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Operation {
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBody>,
    #[serde(default)]
    pub responses: BTreeMap<String, Response>,
    /// Security requirement objects. Presence of any entry means the operation
    /// requires authentication.
    #[serde(default)]
    pub security: Vec<BTreeMap<String, serde_json::Value>>,
}

/// A request/query/path parameter.
#[derive(Debug, Clone, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    #[serde(default)]
    pub required: bool,
    pub description: Option<String>,
    pub schema: Option<Schema>,
}

/// Request body wrapper.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestBody {
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub content: BTreeMap<String, MediaType>,
}

/// A response for a given status code.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Response {
    pub description: Option<String>,
    #[serde(default)]
    pub content: BTreeMap<String, MediaType>,
}

/// A media type entry (e.g. `application/json`) carrying a schema.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct MediaType {
    pub schema: Option<Schema>,
}

/// An OpenAPI schema object (the subset we need).
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Schema {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    pub format: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    #[serde(rename = "enum", default)]
    pub enum_values: Vec<serde_json::Value>,
    #[serde(default)]
    pub properties: BTreeMap<String, Schema>,
    #[serde(default)]
    pub required: Vec<String>,
    pub items: Option<Box<Schema>>,
    /// `additionalProperties` may be a bool or a schema; we only need to know
    /// whether it is a schema (for map types).
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<Box<AdditionalProperties>>,
    /// Composition keywords. We treat any of these as "opaque JSON".
    #[serde(rename = "allOf", default)]
    pub all_of: Vec<Schema>,
    #[serde(rename = "oneOf", default)]
    pub one_of: Vec<Schema>,
    #[serde(rename = "anyOf", default)]
    pub any_of: Vec<Schema>,
}

/// `additionalProperties` can be a boolean or a nested schema.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Bool(bool),
    Schema(Schema),
}

impl Schema {
    /// The schema name a `$ref` points to (the last path segment).
    pub fn ref_name(&self) -> Option<&str> {
        self.reference
            .as_deref()
            .and_then(|r| r.rsplit('/').next())
    }
}
