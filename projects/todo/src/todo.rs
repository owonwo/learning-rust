use std::fmt::Display;
use console::style;

use crate::database::Database;

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

struct TodoItem {
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



/** Add the new item to the database **/
pub fn add_item(string: Vec<String>) -> () {
    let database = Database::new("./todos.db");

    match database {
        Ok(mut db) => {
            for i in string {
                let item = TodoItem::new(i, false);
                let status_string = item.status.to_string();

                db.set(&item.text, &status_string);
            }

            db.save_changes();
        }
        Err(_) => {
            println!("Database file not found");
        }
    }
}

pub fn list_item() -> () {
    let database = Database::new("./todos.db");

    match database {
        Ok(db) => {
            let entries: Vec<_> = db.into();
            for (text, status) in entries {
                let item_is_checked = status.parse::<bool>().unwrap_or(false);
                if item_is_checked {
                    println!("{}", style(text).red().strikethrough());
                } else {
                    println!("{}", text);
                }
            }
        }
        Err(_) => todo!(),
    }
}