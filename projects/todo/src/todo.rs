use std::fmt::Display;
use console::style;

use crate::{database::{sqlite_database::SQLDatabase}, todo_manager::TodoManager};

pub enum TodoAction {
    Add,
    Done,
    Remove,
    List,
    Unknown,
    Check,
}

impl TodoAction {
    pub fn guess_command(string: &str) -> TodoAction {
        match string {
            "add" => TodoAction::Add,
            "done" => TodoAction::Done,
            "remove" => TodoAction::Remove,
            "list" => TodoAction::List,
            "check" => TodoAction::Check,
            _ => TodoAction::Unknown,
        }
    }
}

pub struct TodoItem {
    pub text: String,
    pub status: bool,
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
    for i in &string {
        let item = TodoItem::new(i.to_owned(), false);
        let db = SQLDatabase::init().expect("Unable to communicate with Database");

        let _ = db.add_item(item);
        // strategy.set(&item.text, &status_string);
    }

    if &string.len() > &2 {
        println!("✨ {} Todos added", string.len());
    } else {
        println!("✨ Todo added");
    }
    
    return ();
}

pub fn list_item() -> () {
    let database = SQLDatabase::init();

    if let Ok(db) = database {
        let records = db.get_items();
        // let entries = // strategy.get_items()

        match records {
            Ok(entries) => {
                for item in entries {
                    let item_is_checked = item.status;

                    if item_is_checked {
                        println!("{}", style(item.text).red().strikethrough());
                    } else {
                        println!("{}", item.text);
                    } 
                }
            }
            Err(err) => panic!("{}", err)
        }
    }
}

pub fn check_item(item: String) -> () {
    // strategy.check()
    let db = SQLDatabase::init().expect("No database connection established");
    db.check_item(&item);

    return ()
}