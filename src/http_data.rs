use crate::{
    comment::{Comment, CommentsHolder, ValueType},
    schema::{self, Path},
};

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
    content_type: Option<String>,
    auth: Option<String>,
    comments: CommentsHolder,
}

impl Default for HttpData {
    fn default() -> HttpData {
        HttpData {
            method: HttpMethod::GET,
            path: "".to_owned(),
            host: "host: {{HTTP_HOST}}".to_owned(),
            comments: CommentsHolder {
                query: Vec::new(),
                parameters: Vec::new(),
                body: Vec::new(),
            },
            auth: None,
            content_type: None,
        }
    }
}

fn is_query_param(subs: &str) -> bool {
    // TODO: More concrete solution
    return subs.starts_with("{") && subs.contains("}");
}

#[derive(Clone)]
pub struct Names {
    pub folders: Vec<String>,
    pub file_path: String,
    pub file_name: String,
    pub http_path: String,
}

impl Names {
    pub fn new(value: &String) -> Self {
        let mut splits: Vec<String> = value
            .split('/')
            .map(|split| String::from(split.clone()))
            .collect();

        if splits.len() == 0 {
            panic!("Invalid endpoint name");
        }

        // remove query params from the splits
        splits.retain(|split| !is_query_param(split));

        let file_path = splits.join("/");
        let file_name = splits.pop().unwrap().clone();
        let folders = splits.to_vec().iter().fold(Vec::new(), |mut acc, folder| {
            if acc.len() == 0 {
                acc.push(folder.clone());
            } else {
                let mut last_folder = acc.last().unwrap().clone();
                last_folder.push_str("/");
                last_folder.push_str(folder);
                acc.push(last_folder.clone());
            }
            return acc;
        });

        return Names {
            file_name,
            folders,
            file_path,
            http_path: value.clone(),
        };
    }
}

impl HttpData {
    pub fn new(names: &Names, endpoint_info: &Path, method: &schema::HttpMethod) -> Self {
        let mut data: HttpData = Default::default();

        // convert raw schema method "get" -> "GET"
        data.method = HttpMethod::from(method.to_owned());
        data.path = names.http_path.to_owned();

        if let Some(parameters) = &endpoint_info.parameters {
            // get parameters
            for params in parameters {
                let comment = Comment {
                    r#type: ValueType::String,
                    name: params.name.clone(),
                    required: params.required,
                    default: params.default.clone(),
                };

                match params.r#in.as_ref() {
                    "query" => data.comments.query.push(comment),
                    "path" => data.comments.parameters.push(comment),
                    _ => (),
                }
            }
        }

        // TODO: Check authorization & body schema

        return data;
    }

    /// Converts HttpData struct to formatted string
    pub fn get_formatted(&self) -> String {
        let mut output: Vec<&str> = Vec::new();

        // COMMENTS
        let comments = self.comments.get_formatted();
        if comments.len() > 0 {
            output.push(&comments);
        }

        // METHOD & PATH
        let path_and_method = format!("{} {}", self.method.to_string(), self.path);
        output.push(&path_and_method);

        // HOST
        output.push(&self.host);

        // CONTENT-TYPE
        if let Some(content_type) = &self.content_type {
            output.push(content_type);
        }

        // AUTH
        if let Some(auth) = &self.auth {
            output.push(auth);
        }

        return output.join("\n");
    }
}
