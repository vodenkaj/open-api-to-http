use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize)]
pub struct Path {
    pub responses: HashMap<i32, Response>,
    pub parameters: Option<Vec<Parameters>>,
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    pub r#in: String,
    pub schema: Value,
    pub name: String,
    pub required: Option<bool>,
    pub default: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub content: Option<HashMap<String, Value>>,
}

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
pub struct Schema {
    pub paths: HashMap<String, HashMap<HttpMethod, Path>>,
}

impl Schema {

    /// Creates schema and validates it
    pub fn new(path: &String) -> Schema {
        let data = fs::read_to_string(path).unwrap();
        let res = from_str(&data);

        match res {
            Ok(schema) => schema,
            Err(err) => panic!("{:?}", err),
        }
    }
}
