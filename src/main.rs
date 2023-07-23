mod utils {
    pub mod arguments;
    pub mod info;
    pub mod filesystem;
}

mod project_manager;

use utils::arguments::Arguments;
use utils::info::Info;

use project_manager::ProjectManager;

use std::io;
use std::process::exit;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::layout::{Layout, Constraint, Direction};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn help_menu() {
    let name: &str = &Info::get_name();
    let version = &Info::get_version();
    println!("{} - Project Manager CLI", name);
    println!("{} Version: {}\n", name, version);

    println!("Usage: ./{} [OPTIONS]", name);
    println!("-h, --help\t\t\tPrints help menu");
    println!("-v, --version\t\t\tPrints version information");
    println!("create <project_name>\t\tCreate a new project");
    println!("delete <project_name>\t\tDelete a project");
    println!("list, ls\t\t\tList all projects");
    exit(0);
}

fn main() -> Result<(), io::Error> {
    let args = Arguments::new();
    let pm_manager: ProjectManager = ProjectManager::new();


    if Arguments::has_flag(&args, "-h") || Arguments::has_flag(&args, "--help") {
        help_menu();
    }
    if Arguments::has_flag(&args, "-v") || Arguments::has_flag(&args, "--version") {
        println!("{} version {}", Info::get_name(), Info::get_version());
    }
    if Arguments::has_flag(&args, "create") {
        pm_manager.create(Arguments::get_value(&args, "create"));
    }
    if Arguments::has_flag(&args, "delete") {
        pm_manager.delete(Arguments::get_value(&args, "delete"));
    }
    if Arguments::has_flag(&args, "list") || Arguments::has_flag(&args, "ls") {
        match display_list("Project List", pm_manager.list_name()) {
            Ok(index) => {
                if index != -1 {
                    match display_list("Choice your IDE", pm_manager.ide_list()) {
                        Ok(ide) => {
                            if ide != -1 {
                                pm_manager.open(index, ide);
                            }
                        },
                        Err(_) => {
                            exit(1);
                        }
                    }
                }
                exit(1);
            },
            Err(_) => {
                exit(1);
            }
        }
    }
    Ok(())
}

 fn display_list(title: &str, data: Vec<String>) -> Result<i32, io::Error> {
    // Create a Termion backend for the terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // List items to display in the TUI
    let items = data;

    // Current selected index
    let mut selected_item: i32 = 0;
    let mut selected_index = 0;

    // Create a ListState to keep track of the list's state
    let mut list_state = ListState::default();
    list_state.select(Some(selected_index));
    terminal.clear()?;


    loop {
        terminal.draw(|f| {
            // Create a layout with a single centered block to display the list
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0)].as_ref())
                .split(f.size());

            // Create a list widget to display the items
            let items_list = List::new(
                items.iter()
                    .enumerate()
                    .map(|(i, item)| {
                        if i == selected_index {
                            ListItem::new(item.to_string()).style(tui::style::Style::default().fg(tui::style::Color::Yellow))
                        } else {
                            ListItem::new(item.to_string())
                        }
                    })
                    .collect::<Vec<_>>(),
            ).block(Block::default().borders(Borders::ALL).title(title));

            // Render the list widget with the correct ListState
            f.render_stateful_widget(items_list, chunks[0], &mut list_state);
        })?;

        // Wait for user input
        if let Some(key_event) = io::stdin().keys().next() {
            match key_event.unwrap() {
                termion::event::Key::Char('q') => {
                    selected_item = -1;
                    break;
                }, // Exit the loop when 'q' is pressed
                termion::event::Key::Up => {
                    if selected_index > 0 {
                        selected_index -= 1;
                        selected_item -= 1;
                    }
                }
                termion::event::Key::Down => {
                    if selected_index < items.len() - 1 {
                        selected_index += 1;
                        selected_item += 1;
                    }
                }
                termion::event::Key::Char('\n') => {
                    // Action to be performed when "Enter" key is pressed
                    terminal.clear()?;
                    break;
                }
                _ => {}
            }

            // Update the ListState with the new selected index
            list_state.select(Some(selected_index));

            // Clear the terminal before drawing again
            terminal.clear()?;
        }
    }
    Ok(selected_item)
}