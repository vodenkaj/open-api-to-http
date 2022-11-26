use std::{env, path::Path};

use crate::{
    http_data::{HttpData, Names},
    schema::Schema,
    utils::{create_file, create_folders},
};

struct Config {
    file_path: String,
    output_path: String,
}

pub struct Application {
    config: Config,
}

impl Application {

    /// Valides provided parameters, and checkes if schema & folder paths exists.
    pub fn prepare() -> Result<Self, exitcode::ExitCode> {
        let args: Vec<String> = env::args().collect();

        if args.len() != 3 {
            return Err(exitcode::CONFIG);
        }

        let config = Config {
            file_path: args[1].clone(),
            output_path: args[2].clone(),
        };

        if !Path::new(&config.file_path).exists() || !Path::new(&config.output_path).exists() {
            return Err(exitcode::CONFIG);
        }

        let app = Application { config };

        return Ok(app);
    }

    pub fn run(&self) -> Result<(), exitcode::ExitCode> {
        let schema = Schema::new(&self.config.file_path);

        for (endpoint, endpoint_stucture) in schema.paths {
            let names = Names::new(endpoint);

            let mut formatted_data: Vec<String> = Vec::new();
            for (method, endpoint_info) in endpoint_stucture {
                let http_data = HttpData::new(&names, &endpoint_info, &method);
                formatted_data.push(http_data.get_formatted());
            }

            // join all http requests for this endpoint together
            let joined_data = &formatted_data.join("\n\n");
            let file_path = &format!("{}/{}", &self.config.output_path, &names.file_path);

            create_folders(names.folders.to_owned(), &self.config.output_path);
            create_file(joined_data, file_path);
        }

        return Ok(());
    }
}
