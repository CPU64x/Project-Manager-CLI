use std::process::exit;
use crate::utils::{info::Info, filesystem::FileSystem};
use std::process::Command;

pub struct ProjectManager;

impl ProjectManager {
    pub fn new() -> ProjectManager {
        ProjectManager::init();

        ProjectManager
    }

    pub fn init() {
        if !Info::is_initialized() {
            println!("{} - Initializing project...", Info::get_name());
            let path = FileSystem::join_paths(&FileSystem::get_program_data_dir(), &"ProjectManagerCLI".to_string());
            FileSystem::create_folder(&path);
            println!("{} - Project initialized", Info::get_name());
            exit(0);
        }
    }

    pub fn check_project_exists(project_name: &str) -> bool {
        let path = FileSystem::join_paths(&FileSystem::get_program_data_dir(), &"ProjectManagerCLI".to_string());
        let path = FileSystem::join_paths(&path, &project_name.to_string());

        println!("Checking if project exists...");
        println!("Project path: {}", path);

        FileSystem::folder_exists(path)
    }

    pub fn create(&self, project_name: Option<String>) {

        println!("Creating new project...\n");

        match project_name {
            Some(name) => {
                println!("Project name: {}", name);
                let current_path = FileSystem::get_current_path();
                println!("Project path: {}\n", current_path);

                if ProjectManager::check_project_exists(&name) {
                    eprintln!("Project already exists");
                    exit(1);
                }
                println!("Adding project to database...");
                let path: String = FileSystem::join_paths(&FileSystem::get_program_data_dir(), &"ProjectManagerCLI".to_string());
                let final_path: String = FileSystem::join_paths(&path, &name.to_string());
                FileSystem::create_folder(&final_path);

                FileSystem::create_file(FileSystem::join_paths(&final_path, &".linkto".to_string()), FileSystem::get_current_path());

                println!("Project created successfully");

                exit(0);
            },
            None => {
                println!("No project name provided");
                println!("Please provide a project name");
                exit(1);
            }
        }
    }


    // delete project from database
    pub fn delete(&self, project_name: Option<String>) {

        println!("Deleting project...\n");

        match project_name {
            Some(name) => {
                println!("Project name: {}", name);

                if !ProjectManager::check_project_exists(&name) {
                    eprintln!("Project does not exist");
                    exit(1);
                }
                println!("Deleting project from database...");
                let path: String = FileSystem::join_paths(&FileSystem::get_program_data_dir(), &"ProjectManagerCLI".to_string());
                let final_path: String = FileSystem::join_paths(&path, &name.to_string());
                FileSystem::remove_folder(&final_path);

                println!("Project deleted successfully");

                exit(0);
            },
            None => {
                println!("No project name provided");
                println!("Please provide a project name");
                exit(1);
            }
        }
    }

    pub fn list(&self) -> Vec<String> {
        println!("Listing projects...\n");

        let path: String = FileSystem::join_paths(&FileSystem::get_program_data_dir(), &"ProjectManagerCLI".to_string());
        return FileSystem::get_paths_in_folder(&path);
    }

    pub fn list_name(&self) -> Vec<String> {
        let path: String = FileSystem::join_paths(&FileSystem::get_program_data_dir(), &"ProjectManagerCLI".to_string());
        let mut list: Vec<String> = FileSystem::get_paths_in_folder(&path);

        for i in 0..list.len() {
            list[i] = FileSystem::get_folder_name(&list[i]);
        }
        return list;
    }

    // open project from database by index
    pub fn open(&self, index: i32, ide: i32) {
        let projects: Vec<String> = self.list();

        if index < 0 || index >= projects.len() as i32 {
            eprintln!("Invalid index");
            exit(1);
        }
        let file_linkto: &String = &FileSystem::join_paths(&projects[index as usize], &".linkto".to_string());

        let path = FileSystem::read_file(file_linkto);

        println!("Opening project: {}", path);

        Command::new(self.ide_list()[ide as usize].to_string())
            .arg(path)
            .spawn()
            .expect("Failed to open project");

        println!("Project opened successfully")
    }

    pub fn ide_list(&self) -> Vec<String> {
        let projects: Vec<String> = vec![
            "code".to_string(),
            "clion".to_string(),
            "webstorm".to_string(),
            "pycharm".to_string(),
            "intellij".to_string(),
            "datagrip".to_string(),
            "goland".to_string(),
            "phpstorm".to_string(),
            "android-studio".to_string(),
            "rider".to_string()
        ];

        return projects;
    }


}