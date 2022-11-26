use std::io::prelude::*;
use std::{fs, io, path::Path};

fn create_folder_if_not_exists(name: &String) -> Result<(), io::Error> {
    if !Path::new(name).exists() {
        fs::create_dir(name)?;
    }
    Ok(())
}

/// Creates provided folders in order at specified path.
pub fn create_folders(folders: Vec<String>, output_path: &String) {
    let mut path = output_path.to_owned();
    create_folder_if_not_exists(&path).unwrap();
    for folder in folders {
        path.push_str(&format!("/{}", folder));
        create_folder_if_not_exists(&path).unwrap();
    }
}

/// Creates file and writes all provided data.
pub fn create_file(data: &String, path: &String) {
    let mut file = fs::File::create(path).unwrap();

    let res = file.write_all(data.as_bytes());

    match res {
        Ok(res) => res,
        Err(err) => panic!("{}", err),
    }
}
