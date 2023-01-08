use std::fs::OpenOptions;
use std::io::prelude::*;
use std::{fs, io, path::Path};

fn create_folder_if_not_exists(name: &String) -> Result<(), io::Error> {
    if !Path::new(name).exists() {
        fs::create_dir(name)?;
    }
    Ok(())
}

/// Creates provided folders in order at specified path.
pub fn create_folders(folders: &Vec<String>, output_path: &String) {
    for folder in folders {
        let path = format!("{}/{}", output_path, folder);
        create_folder_if_not_exists(&path).unwrap();
    }
}

/// Creates file and writes all provided data.
pub fn create_file(data: &String, path: &String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();


    let res = file.write_all(data.as_bytes());

    match res {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    }
}
