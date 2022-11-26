use std::collections::{HashMap, HashSet};

use crate::schema::{self, Path};

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl From<schema::HttpMethod> for HttpMethod {
    fn from(raw_http_data: schema::HttpMethod) -> Self {
        match raw_http_data {
            schema::HttpMethod::get => HttpMethod::GET,
            schema::HttpMethod::post => HttpMethod::POST,
            schema::HttpMethod::put => HttpMethod::PUT,
            schema::HttpMethod::patch => HttpMethod::PATCH,
            schema::HttpMethod::delete => HttpMethod::DELETE,
        }
    }
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        return format!("{:?}", &self);
    }
}

pub struct HttpData {
    method: HttpMethod,
    path: String,
    host: String,
    query_params: HashMap<String, String>,
    content_type: Option<String>,
    auth: Option<String>,
}

impl Default for HttpData {
    fn default() -> HttpData {
        HttpData {
            content_type: Some("Content-Type:".to_owned()),
            method: HttpMethod::GET,
            path: "".to_owned(),
            host: "Host: {{HTTP_HOST}}".to_owned(),
            auth: Some("Authorization: {{HTTP_AUTH}}".to_owned()),
            query_params: HashMap::new(),
        }
    }
}

fn is_query_param(subs: &str) -> bool {
    // TODO: More concrete solution
    return subs.starts_with("{") && subs.contains("}");
}

pub struct Names {
    pub folders: Vec<String>,
    pub file_path: String,
    http_path: String,
}

impl Names {
    pub fn new(value: String) -> Self {
        let mut splits: Vec<String> = value
            .split('/')
            .map(|split| String::from(split.to_owned()))
            .collect();

        if splits.len() == 0 {
            panic!("Invalid endpoint name");
        }

        let last_split = splits.pop().unwrap();
        splits.retain(|split| !is_query_param(split));

        let mut file_path = splits.join("/");
        let folders = splits.to_vec();

        if is_query_param(&last_split) {
            file_path.push_str(&format!("/{}.http", splits.last().unwrap().to_owned()));
        } else {
            file_path.push_str(&format!("/{}.http", last_split));
        }

        return Names {
            folders,
            file_path,
            http_path: value.to_owned(),
        };
    }
}

impl HttpData {
    pub fn new(names: &Names, endpoint_info: &Path, method: &schema::HttpMethod) -> Self {
        let mut data: HttpData = Default::default();

        // convert raw schema method "get" -> "GET"
        data.method = HttpMethod::from(method.to_owned());
        data.path = names.http_path.to_owned();

        let mut contents: HashSet<String> = HashSet::new();

        // get all possible content types
        for (_status, response) in &endpoint_info.responses {
            if response.content.is_some() {
                for (key, _value) in response.content.as_ref().unwrap() {
                    contents.insert(key.to_owned());
                }
            }
        }

        let mut res = String::new();
        for content_type in contents {
            res.push_str(&format!("{};", content_type));
        }

        data.content_type = Some(format!("Content-Type: {}", res));

        if let Some(parameters) = &endpoint_info.parameters {
            // get parameters
            for params in parameters {
                // query params
                match params.r#in.to_owned().as_str() {
                    "query" => {
                        let mut default = "".to_owned();
                        if params.default.is_some() {
                            default = params.default.as_ref().unwrap().to_owned();
                        }
                        data.query_params.insert(params.name.to_owned(), default);
                    }
                    _ => {}
                }
            }
        }

        return data;
    }

    /// Converts HttpData struct to formatted string
    pub fn get_formatted(&self) -> String {
        let mut params: String = "?".to_owned();
        for (key, value) in self.query_params.to_owned() {
            params.push_str(&format!("{}={}&", key, value));
        }

        return format!(
            "{} {}{}\n{}\n{}\n{}",
            self.method.to_string(),
            self.path,
            params,
            self.host,
            self.content_type.as_deref().unwrap_or_default(),
            self.auth.as_deref().unwrap_or_default(),
        );
    }
}
