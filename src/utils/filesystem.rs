use std::fs;
use std::path::PathBuf;

pub struct FileSystem;

impl FileSystem {
    // create_folder creates a folder in the current directory
    pub fn create_folder(folder_name: &str) -> bool {
        if let Err(err) = fs::create_dir(folder_name) {
            println!("Error creating folder: {}", err);
            return false;
        }
        return true;
    }

    // remove_folder removes a folder in the current directory
    pub fn remove_folder(folder_name: &str) -> bool {
        if let Err(err) = fs::remove_dir_all(folder_name) {
            println!("Error removing folder: {}", err);
            return false;
        }
        return true;
    }

    // get_paths_in_folder returns a vector of paths in a folder
    pub fn get_paths_in_folder(folder_name: &str) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();
        let paths_in_folder = fs::read_dir(folder_name).unwrap();
        for path in paths_in_folder {
            let path = path.unwrap().path();
            let path = path.to_str().unwrap().to_string();
            paths.push(path);
        }
        return paths;
    }

    // get_folder_name returns the name of the folder
    pub fn get_folder_name(folder_path: &str) -> String {
        let path = PathBuf::from(folder_path);
        let folder_name = path.file_name().unwrap().to_str().unwrap().to_string();
        return folder_name;
    }

    // create_file creates a file in the current directory
    pub fn get_program_data_dir() -> String {
        match dirs::data_local_dir() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            None => {
                eprintln!("Error getting program data directory");
                String::new()
            }
        }
    }

    // file_exists checks if a file exists in the current directory
    pub fn folder_exists(folder_name: String) -> bool {
        if let Err(_) = fs::metadata(folder_name) {
            return false;
        }
        return true;
    }

    // join paths
    pub fn join_paths(path1: &str, path2: &str) -> String {
        let mut path = PathBuf::new();
        path.push(path1);
        path.push(path2);
        return path.to_str().unwrap().to_string();
    }

    pub fn get_current_path() -> String {
        let current_path = std::env::current_dir().unwrap();
        return current_path.to_str().unwrap().to_string();
    }

    // create a file and write data to it
    pub fn create_file(file_name: String, data: String) -> bool {
        if let Err(err) = fs::write(file_name, data) {
            println!("Error creating file: {}", err);
            return false;
        }
        return true;
    }

    // read file data
    pub fn read_file(file_name: &str) -> String {
        let data = fs::read_to_string(file_name).unwrap();
        return data;
    }
}