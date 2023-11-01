use crate::{todo::TodoItem, todo_manager::TodoManager};
use rusqlite::{ Connection, Result, Row};

const STORAGE_DIRECTORY: &str = "./storage";

#[derive(Debug)]
pub struct SQLDatabase {
    pub conn: Connection,
}

impl SQLDatabase {
    pub fn init() -> Result<SQLDatabase> {
        let storage_path = format!("{STORAGE_DIRECTORY}");
        let db_file_path = format!("{storage_path}/todo.db");

        ensure_directory_exisits(&storage_path);

        Ok(SQLDatabase {
            conn: Connection::open(db_file_path)?,
        })
    }

    pub fn setup(&self) -> Result<()> {
        let value = self.conn.execute(
            "CREATE TABLE TodoItem (
                id INT,
                status BOOLEAN,
                title VARCHAR(255),
                created_at TIMESTAMP
            )",
            (), // empty list of parameters.
        )?;

        println!("{value}");

        Ok(())
    }
}

impl TodoManager for SQLDatabase {
    fn get_items(&self) -> Result<Vec<TodoItem>, anyhow::Error> {
        let conn = SQLDatabase::init().expect("Unable to establish connection");
        let stmt = conn.conn.prepare("SELECT * FROM TodoItem");

        let value = match stmt {
            Ok(mut stmt_handle) => {
                let rows = stmt_handle.query_map([], |row| item_from_row(row));

                let results: Result<Vec<TodoItem>, rusqlite::Error> = rows?.collect();

                Ok(results?)
            }
            Err(_) => Err(anyhow::anyhow!("Invalid statement provided!")),
        };

        return Ok(value?);
    }

    fn add_item(&self, item: TodoItem) -> Result<(), anyhow::Error> {
        let conn = SQLDatabase::init().expect("Unable to establish connection");

        // TODO: Only save distinct TodoItems
        let status = &item.status.to_status_string();

        let result = conn.conn.execute(
            "INSERT INTO TodoItem (status, title) VALUES (:status, :title);",
            &[(":title", &item.text), (":status", &status)],
        );

        return match result {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow::anyhow!("Unable to save todo item {}", err)),
        };
    }

    fn check_item(&self, item_name: &String) -> std::result::Result<(), anyhow::Error> {
        let conn = SQLDatabase::init().expect("Unable to establish connection");
        let params = &[(":title", &item_name)];
        let find_query = conn
            .conn
            .query_row("SELECT * FROM TodoItem where title = :title", params, |r| {
                item_from_row(r)
            });

        match find_query {
            Ok(mut todo_item) => {
                // update status
                println!("Before {}", todo_item);
                todo_item.status = true;
                println!("After {}", todo_item);

                // write to storage
                let update_query = conn.conn.execute(
                    "UPDATE TodoItem SET title = :title, status = :status WHERE title = :title",
                    &[(":title", &todo_item.text), (":status", &todo_item.status.to_status_string())],
                );

                // ensure write was successful
                if let Err(err) = update_query {
                    panic!("Something went wrong. We're unable to change status {}. {}", item_name, err)
                }
            }
            Err(err) => {
                panic!("{}", err)
            }
        }

        Ok(())
    }
}

fn ensure_directory_exisits(dir_name: &str) -> () {
    let handle = std::fs::read_dir(dir_name);

    if let Err(err) = handle {
        let msg = format!("We couldn't create the {dir_name} directory. {err}");
        std::fs::create_dir(dir_name).expect(&msg);
    }
}

fn item_from_row(p: &Row<'_>) -> std::result::Result<TodoItem, rusqlite::Error> {
    let status: bool = p.get(3)?;
    let name: String = p.get(1)?;

    Ok(TodoItem::new(name, status))
}

trait TodoStatus {
    fn to_status_bool(&self) -> bool {
        panic!("Oops! you convert type to `bool`");
    }

    fn to_status_string(&self) -> String {
        panic!("Oops! you convert type to `String`");
    }
}

impl TodoStatus for bool {
    fn to_status_bool(&self) -> bool {
        todo!("Not allowed")
    }

    fn to_status_string(&self) -> String {
        return match self {
            true => "1".to_owned(),
            false => "0".to_owned(),
        };
    }
}

impl TodoStatus for str {
    fn to_status_bool(&self) -> bool {
        return match self {
            "1" => true,
            "0" => false,
            "true" => true,
            "false" => false,
            _ => false
        };
    }
}