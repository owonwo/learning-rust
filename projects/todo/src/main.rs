mod todo;
mod todo_manager;
pub mod database;

use console::style;
use todo::{TodoAction, list_item, add_item, check_item};

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let command = TodoAction::guess_command(&arguments[0]);
    let other_commands = arguments.into_iter().skip(1).collect::<Vec<_>>();

    match command {
        TodoAction::List => list_item(),
        TodoAction::Add => add_item(other_commands),
        TodoAction::Check => {
            for todo_item in other_commands {
                check_item(todo_item);
            }
        },
        TodoAction::Done => todo!("Handle Done command"),
        TodoAction::Remove => todo!("Handle Remove command"),
        TodoAction::Unknown => println!("Invalid command provided!"),
    }

    println!("{}", style("🚀 Done!").green());
}