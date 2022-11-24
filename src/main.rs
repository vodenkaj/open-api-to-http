use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::{
    collections::{HashMap, HashSet},
    env, fs,
    io::{self, Write},
    path,
};

#[derive(Debug)]
struct Error {
    message: String,
}

struct Config {
    file_path: String,
    output_path: String,
}

#[derive(Serialize, Deserialize)]
struct Path {
    pub responses: HashMap<i32, Response>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    pub content: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
enum HttpMethod {
    get,
    post,
    put,
    delete,
    patch,
}

impl HttpMethod {
    fn get_value(&self) -> String {
        return format!("{:?}", &self).to_uppercase();
    }
}

#[derive(Serialize, Deserialize)]
struct Schema {
    paths: HashMap<String, HashMap<HttpMethod, Path>>,
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

    let schema = get_schema_from_file(&config);
    process_schema(schema, config);
}

fn process_schema(schema: Schema, config: Config) {
    for (endpoint, endpoint_info) in schema.paths {
        let names = get_names(&endpoint);
        create_folders(names.folders.to_owned(), &config.output_path);
        create_files(endpoint_info, names, &config.output_path).expect("error not happening");
    }
}

fn get_schema_from_file(Config { file_path, .. }: &Config) -> Schema {
    let path = path::Path::new(&file_path);
    if !path::Path::exists(path) {
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

fn create_files(
    paths: HashMap<HttpMethod, Path>,
    names: Names,
    output_path: &String,
) -> Result<(), io::Error> {
    for (method, path) in paths.iter() {
        let mut file = fs::File::create(format!("{}/{}", output_path, &names.abs_path))?;
        let http_data = create_http_data(path, method, &names.http_path);
        file.write_all(http_data.get_formatted().as_bytes());
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

impl HttpData {
    fn get_formatted(&self) -> String {
        return format!(
            "{}\n{}\n{}\n{}",
            self.method,
            self.host,
            self.content_type.as_deref().unwrap_or_default(),
            self.auth.as_deref().unwrap_or_default(),
        );
    }
}

fn create_http_data(path: &Path, method: &HttpMethod, http_path: &String) -> HttpData {
    let mut data: HttpData = Default::default();
    data.method = format!("{} {}", method.get_value(), http_path);

    let mut contents: HashSet<String> = HashSet::new();

    for (_status, response) in &path.responses {
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
    return data;
}

fn create_folder_if_not_exists(name: &String) -> Result<(), io::Error> {
    if !path::Path::new(name).exists() {
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

    let mut folders = splits.to_vec();
    let file = format!("{}.http", splits.last().unwrap().to_owned());

    if folders.len() > 1 {
        folders.pop();
    }

    let abs_path = format!("{}/{}", folders.join("/"), file);

    return Names {
        folders,
        file,
        abs_path,
        http_path: endpoint.to_owned(),
    };
}
