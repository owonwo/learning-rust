use std::io::{Error, ErrorKind};
use database::Database;

mod database;

fn main() -> Result<(), std::io::Error> {
    let args = std::env::args();
    let arguments = args.skip(1).collect::<Vec<_>>();

    if arguments.len() != 2 {
        return Err(Error::new(ErrorKind::Other, "Argument required"));
    };

    // decode serialized kv pairs from database file
    let file_name = String::from("kvstore.db");
    let mut database = Database::new(&file_name)?;

    database.set(&arguments[0], &arguments[1]);
    database.save_changes();

    return Ok(());
}
