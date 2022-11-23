use serde_json::{from_str, Value};
use std::{
    env, fs,
    io::{self, Write},
    path::Path,
};

#[derive(Debug)]
struct Error {
    message: String,
}

struct Config {
    file_path: String,
    output_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Missing arguments")
    }

    let config = Config {
        file_path: args[1].clone(),
        output_path: args[2].clone(),
    };

    let schema: Value = get_schema_from_file(&config);
    process_schema(schema, config);
}

fn process_schema(schema: Value, config: Config) {
    let path = schema["paths"].clone();
    let host: String = match &schema["host"] {
        Value::Null => match &schema["servers"] {
            Value::Array(servers) => match &servers[0] {
                Value::Object(server) => match &server["url"] {
                    Value::String(url) => url.clone(),
                    _ => panic!(
                        "Server url is supposed to be string, not: {:?}",
                        servers[0]["url"].to_string()
                    ),
                },
                _ => panic!(
                    "Server is supposed to be object, not: {:?}",
                    servers[0].to_string()
                ),
            },
            _ => panic!(
                "Servers is supposed to be array, not: {:?}",
                schema["servers"].to_string()
            ),
        },
        Value::String(host) => host.clone(),
        _ => panic!("Host or Servers must be present"),
    };

    if let Value::Object(path) = path {
        for (endpoint, endpoint_info) in path {
            let names = get_names(&endpoint);
            create_folders(names.folders.to_owned(), &config.output_path);
            create_files(endpoint_info, names, &config.output_path).expect("error not happening");
        }
    } else {
        panic!(
            "Expected type of Object at schema paths, got: {:?}",
            path.as_str()
        );
    }
}

fn get_schema_from_file(Config { file_path, .. }: &Config) -> Value {
    let path = Path::new(&file_path);
    if !Path::exists(path) {
        panic!("Could not find schema at {}.", file_path);
    }

    let data = fs::read_to_string(file_path.clone()).unwrap();
    let res = from_str(&data);

    match res {
        Ok(schema) => schema,
        Err(err) => panic!("Could not convert schema to JSON {:?}", err.to_string()),
    }
}

struct Names {
    folders: Vec<String>,
    file: String,
    http_path: String,
    abs_path: String,
}

fn create_folders(folders: Vec<String>, output_path: &String) {
    let mut path = String::new();
    for folder in folders {
        if path.len() > 0 {
            create_folder_if_not_exists(&format!("{}/{}/{}", output_path, path, folder)).unwrap();
        } else {
            create_folder_if_not_exists(&format!("{}/{}", output_path, folder)).unwrap();
        }
        path.push_str(&folder);
    }
}

fn create_files(path: Value, names: Names, output_path: &String) -> Result<(), io::Error> {
    if let Value::Object(path) = path {
        for (method, info) in path.iter() {
            let mut file = fs::File::create(format!("{}/{}", output_path, &names.abs_path))?;
            let res = create_http_data(info, method, &names.http_path);
            let http_data = match res {
                Ok(res) => res,
                Err(error) => panic!("Error creating http_data: {:?}", error.message),
            };
            let formatted = format!(
                "{}\n{}\n{}\n{}",
                http_data.method,
                http_data.host,
                http_data.content_type.unwrap_or_default(),
                http_data.auth.unwrap_or_default(),
            );
            file.write_all(formatted.as_bytes());
        }
    }
    Ok(())
}

struct HttpData {
    method: String,
    host: String,
    content_type: Option<String>,
    auth: Option<String>,
}

impl Default for HttpData {
    fn default() -> HttpData {
        HttpData {
            content_type: Some("Content-Type:".to_owned()),
            method: "GET".to_owned(),
            host: "Host: {{HTTP_HOST}}".to_owned(),
            auth: Some("Authorization: {{HTTP_AUTH}}".to_owned()),
        }
    }
}

fn create_http_data(info: &Value, method: &String, http_path: &String) -> Result<HttpData, Error> {
    if let Value::Object(info) = info {
        let mut data: HttpData = Default::default();
        data.method = format!("{} {}", method.clone().to_uppercase(), http_path);

        let responses = info["responses"].as_object();
        if let Some(responses) = responses {
            if responses.contains_key("200") {
                let success_response = responses["200"].as_object().unwrap();
                if success_response.contains_key("content") {
                    let content = success_response["content"].as_object().unwrap();
                    data.content_type = std::option::Option::Some(format!(
                        "Content-Type: {}",
                        &content.keys().next().unwrap()
                    ))
                }
            }
        }

        return Ok(data);
    }
    Err(Error {
        message: format!(
            "Schema value expected to be Object, found {}",
            info.to_string()
        ),
    })
}

fn create_folder_if_not_exists(name: &String) -> Result<(), io::Error> {
    if !Path::new(name).exists() {
        fs::create_dir(name)?;
    }
    Ok(())
}

fn is_query_param(subs: &str) -> bool {
    return subs.starts_with("{") && subs.contains("}");
}

fn get_names(endpoint: &String) -> Names {
    let mut splits: Vec<String> = endpoint
        .split('/')
        .map(|split| String::from(split.to_owned()))
        .collect();

    splits.retain(|split| !is_query_param(split));

    if splits.len() == 0 {
        panic!("Invalid endpoint name");
    }

    let file = format!("{}.http", splits.pop().unwrap().to_owned());
    let folders = splits.to_vec();

    let abs_path = format!("{}/{}", folders.join("/"), file);

    return Names {
        folders,
        file,
        abs_path,
        http_path: endpoint.to_owned(),
    };
}
