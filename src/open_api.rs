use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

/// Describes a single API operation on a path.
/// ref: https://spec.openapis.org/oas/v3.1.0#operation-object
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub responses: HashMap<i32, Response>,
    pub parameters: Option<Vec<Parameters>>,
    pub request_body: Option<RequestBody>,
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    pub r#in: String,
    pub schema: Value,
    pub name: String,
    pub required: Option<bool>,
    pub default: Option<String>,
}

/// Describes a single request body.
/// ref: https://spec.openapis.org/oas/v3.1.0#request-body-object
#[derive(Serialize, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: HashMap<String, MediaType>,
    pub required: Option<bool>,
}

/// Each Media Type Object provides schema for the media type identified by its key.
/// ref: https://spec.openapis.org/oas/v3.1.0#mediaTypeObject
#[derive(Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Schema,
}

/// The Schema Object allows the definition of input and output data types. These types can be objects, but also primitives and arrays.
/// This object is a superset of the JSON Schema Specification Draft 2020-12.
/// ref: https://spec.openapis.org/oas/v3.1.0#schema-object
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Object(Object),
    AllOf { allOf: Vec<Object> },
    AnyOf { anyOf: Vec<Object> },
    OneOf { oneOf: Vec<Object> },
    Not { not: Vec<Object> },
}

impl Schema {
    pub fn get_all_types(&self) -> HashSet<PrimitiveType> {
        let mut known_types = HashSet::new();

        // TODO: Handle those properly
        match self {
            Schema::Object(obj) => {
                known_types.insert(obj.r#type.clone());
            }
            Schema::AllOf { allOf } => {
                allOf.iter().for_each(|obj| {
                    known_types.insert(obj.r#type.clone());
                });
            }
            Schema::AnyOf { anyOf } => {
                anyOf.iter().for_each(|obj| {
                    known_types.insert(obj.r#type.clone());
                });
            }
            Schema::OneOf { oneOf } => {
                oneOf.iter().for_each(|obj| {
                    known_types.insert(obj.r#type.clone());
                });
            }
            Schema::Not { not } => {
                not.iter().for_each(|obj| {
                    known_types.insert(obj.r#type.clone());
                });
            }
        }

        return known_types;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub properties: Option<HashMap<String, Schema>>,
    pub required: Option<Vec<String>>,
    pub r#type: PrimitiveType,
}

/// ref: https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-00#section-6.1.1
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PrimitiveType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
}

impl ToString for PrimitiveType {
    fn to_string(&self) -> String {
        return format!("{:?}", &self);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub content: Option<HashMap<String, Value>>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HttpMethod {
    get,
    post,
    put,
    delete,
    patch,
}

impl HttpMethod {
    pub fn get_value(&self) -> String {
        return format!("{:?}", &self).to_uppercase();
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenApi {
    pub paths: HashMap<String, HashMap<HttpMethod, Operation>>,
}

impl OpenApi {
    /// Creates schema and validates it
    pub fn new(path: &String) -> OpenApi {
        let data = fs::read_to_string(path).unwrap();
        let res = from_str(&data);

        match res {
            Ok(schema) => schema,
            Err(err) => panic!("{:?}", err),
        }
    }
}
