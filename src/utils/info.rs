use std::env;

use crate::utils::filesystem::FileSystem;

pub struct Info;

impl Info {
    pub fn get_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    pub fn get_name() -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    pub fn is_initialized() -> bool {
        let path = FileSystem::join_paths(&FileSystem::get_program_data_dir(), &"ProjectManagerCLI".to_string());
        return FileSystem::folder_exists(path);
    }
}