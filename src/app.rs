use unwrap_or::{unwrap_err_or, unwrap_ok_or};

use crate::{
    http_data::{HttpData, Names},
    open_api::OpenApi,
    utils::{create_file, create_folders},
};
use std::{
    collections::{HashMap, HashSet},
    io::Read,
};
use std::{env, path::Path};

struct Config {
    file_path: String,
    output_path: String,
}

pub struct Application {
    config: Config,
}

impl Application {
    /// Validates provided parameters, and checks whenever schema & folder paths exists.
    pub fn prepare() -> Result<Self, exitcode::ExitCode> {
        let args: Vec<String> = env::args().collect();

        unwrap_err_or!(get_argument(&args, &String::from("help"), &false), _, {
            println!("Usage:");
            println!("  open-api-to-http --output PATH --schema PATH");
            println!("      generates http files from provided OpenAPI schema.");
            return Err(exitcode::OK);
        });

        let file_path = unwrap_ok_or!(get_argument(&args, &String::from("schema"), &true), _, {
            eprintln!("Schema path argument is missing!");
            return Err(exitcode::CONFIG);
        });
        let output_path = unwrap_ok_or!(get_argument(&args, &String::from("output"), &true), _, {
            eprintln!("Output path argument is missing!");
            return Err(exitcode::CONFIG);
        });

        let config = Config {
            file_path,
            output_path,
        };

        if !Path::new(&config.file_path).exists() {
            eprintln!("Schema file was not found at {}", config.file_path);
            return Err(exitcode::CONFIG);
        }

        let output_dir = Path::new(&config.output_path);
        if !output_dir.exists() {
            eprintln!("Output folder was not found at {}", config.output_path);
            return Err(exitcode::CONFIG);
        }

        if !Path::read_dir(&output_dir).unwrap().next().is_none() {
            let mut buffer = [0; 1];
            let mut reader = std::io::stdin();

            println!("Output folder is not empty, this could potetionally delete some files, do you want to continue?");

            reader.read_exact(&mut buffer).unwrap();
            let answer = (buffer[0] as char).to_lowercase().next().unwrap();
            if answer != 'y' {
                println!("Aborting.");
                return Err(exitcode::USAGE);
            }
        }

        let app = Application { config };

        return Ok(app);
    }

    pub fn run(&self) -> Result<(), exitcode::ExitCode> {
        let schema = OpenApi::new(&self.config.file_path);
        let mut endpoints_map = HashMap::<String, (Vec<String>, Names)>::new();
        let mut folder_map = HashSet::new();

        for (path_name, endpoint_stucture) in schema.paths {
            let mut formatted_data = Vec::new();
            let names = Names::new(&path_name);

            for (method, endpoint_info) in endpoint_stucture {
                let http_data = HttpData::new(&names, &endpoint_info, &method);
                formatted_data.push(http_data.get_formatted());
            }

            if endpoints_map.contains_key(&names.file_path) {
                endpoints_map
                    .get_mut(&names.file_path)
                    .unwrap()
                    .0
                    .append(&mut formatted_data);
            } else {
                endpoints_map.insert(names.file_path.clone(), (formatted_data, names.clone()));
            }

            folder_map.extend(names.folders.clone());

            create_folders(&names.folders, &self.config.output_path);
        }

        // create files for all the endpoints
        for (_, (data, names)) in endpoints_map {
            let final_file_path;
            if data.len() > 1 {
                // if there are more endpoints with same file_path, create folder for them
                create_folders(
                    &Vec::from([names.file_path.clone()]),
                    &self.config.output_path,
                );
                final_file_path = format!("{}/{}.http", &names.file_path, &names.file_name);
            } else if folder_map.contains(&names.file_path) {
                // folder with same name as this file would have exist,
                // place this file into the existing file
                final_file_path = format!("{}/{}.http", &names.file_path, &names.file_name);
            } else {
                final_file_path = format!("{}.http", &names.file_path);
            }

            create_file(
                &data.join("\n\n"),
                &format!("{}{}", &self.config.output_path, &final_file_path),
            );
        }

        return Ok(());
    }
}

fn get_argument(args: &Vec<String>, name: &String, with_value: &bool) -> Result<String, ()> {
    let arg = args
        .iter()
        .position(|x| x.starts_with("--") && x.eq(&format!("--{}", name)));

    if arg.is_none() || (*with_value && arg.unwrap() == args.len() - 1) {
        return Err(());
    }

    if !with_value {
        return Ok(name.clone());
    } else if args[arg.unwrap() + 1].starts_with("--") {
        return Err(());
    }

    return Ok(args[arg.unwrap() + 1].clone());
}
