# Project Manager CLI

## What we need to do to make it work

## Requirements : 
Have rust installed.


## Installation 

### Run installation script (linux/macOS)

> ./install.sh

## Dev 

## Build Project Manager CLI
> cargo build

## Run Project (without build) (only for test)
> cargo run -- [FLAGS]

## Compatibility

| Operating System         | Version needed |
| :---:                    | :---:          |
| Windows 10/11            |     1.0 |
| MacOS / Linux            |     1.0 |

## Executable Name

> pm

### Help Menu

> pm -h

```text
pm - Project Manager CLI
pm Version: 1.1.0

Usage: ./pm [OPTIONS]
-h, --help                      Prints help menu
-v, --version                   Prints version information
create <project_name>           Create a new project
delete <project_name>           Delete a project
list, ls                        List all projects
```

### Select a project to open

> pm list

#### Select Your project

![Alt text](docs/select_project.jpg?raw=true "Project Manager CLI / Select Project")

Press :  
 - [Enter]: To select the project
 - [q]: To quit.

#### Select Your IDE

![Alt text](docs/select_ide.jpg?raw=true "Project Manager CLI / Select IDE")

Press :  
 - [Enter]: To select the project
 - [q]: To quit.

## Copyrights

```text
Copyrights EQ-0 - 2023, All Rights Reserved

   __ ___________________            _______    __   
  / / \_   _____/\_____  \           \   _  \   \ \  
 / /   |    __)_  /  / \  \   ______ /  /_\  \   \ \ 
 \ \   |        \/   \_/.  \ /_____/ \  \_/   \  / / 
  \_\ /_______  /\_____\ \_/          \_____  / /_/  
              \/        \__>                \/       
```
