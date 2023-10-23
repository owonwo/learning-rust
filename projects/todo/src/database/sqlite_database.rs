use rusqlite::{Connection, Result};
use crate::{todo_manager::{TodoManager}, todo::TodoItem};

const STORAGE_DIRECTORY: &str = "./storage";

#[derive(Debug)]
pub struct SQLDatabase {
    pub conn: Connection
}

impl SQLDatabase {
    
    pub fn init() -> Result<SQLDatabase> {
        let storage_path = format!("{STORAGE_DIRECTORY}");
        let db_file_path = format!("{storage_path}/todo.db");

        ensure_directory_exisits(&storage_path);

        Ok(SQLDatabase {
            conn: Connection::open(db_file_path)?
        })
    }

    pub fn setup(&self) -> Result<()>  {
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
    fn get_items(&self) -> Result<Vec<(String, String)>, anyhow::Error> {
        let conn = SQLDatabase::init().expect("Unable to establish connection");
        let stmt = conn.conn.prepare("SELECT id, status, title FROM TodoItem");

        let value = match stmt {
            Ok(mut stmt_handle) => {
                let rows = stmt_handle.query_map([], |p| {
                    Ok((p.get::<_, String>(1)?, p.get::<_, String>(2)?, ))
                });

                let results: Result<Vec<(String, String)>, rusqlite::Error> = rows?.collect();

                Ok(results?)
            },
            Err(_) => Err(anyhow::anyhow!("Invalid statement provided!")),
        };
        
        return Ok(value?);
    }

    fn add_item(&self, item: TodoItem) -> Result<(), anyhow::Error> {
    //     conn.conn.execute("
    //     INSERT INTO TodoItem (id, status, title, created_at)
    //     VALUES (1, true, {title}, now());
    // ", );
        todo!()
    }
}

fn ensure_directory_exisits(dir_name: &str) -> () {
    let handle = std::fs::read_dir(dir_name);

    if let Err(err) = handle {
        let msg = format!("We couldn't create the {dir_name} directory. {err}");
        std::fs::create_dir(dir_name)
            .expect(&msg);
    }
}