use std::fmt::Display;
use console::style;

use crate::{database::{file_database::FileDatabase, sqlite_database::SQLDatabase}, todo_manager::TodoManager};

pub enum TodoAction {
    Add,
    Done,
    Remove,
    List,
    Unknown,
}

impl TodoAction {
    pub fn guess_command(string: &str) -> TodoAction {
        match string {
            "add" => TodoAction::Add,
            "done" => TodoAction::Done,
            "remove" => TodoAction::Remove,
            "list" => TodoAction::List,
            _ => TodoAction::Unknown,
        }
    }
}

pub struct TodoItem {
    text: String,
    status: bool,
}

impl TodoItem {
    pub fn new(name: String, status: bool) -> TodoItem {
        return TodoItem {
            text: name,
            status: status || false,
        };
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TodoItem(title={}, status={})", self.text, self.status)
    }
}

/** Add the new item to the test_1 **/
pub fn add_item(string: Vec<String>) -> () {
    for i in string {
        let item = TodoItem::new(i, false);
        let status_string = item.status.to_string();

        // strategy.set(&item.text, &status_string);
    }
}

pub fn list_item() -> () {
    let database = SQLDatabase::init();

    if let Ok(db) = database {
        let records = db.get_items();
        // let entries = // strategy.get_items()

        match records {
            Ok(entries) => {
                for (status, text) in entries {
                    let item_is_checked = status.parse::<bool>().unwrap_or(false);
                    if item_is_checked {
                        println!("{}", style(text).red().strikethrough());
                    } else {
                        println!("{}", text);
                    }        
                }
            }
            Err(err) => panic!("{}", err)
        }
    }
}
