use console::style;
mod database;
mod todo;

use todo::{TodoAction, list_item, add_item};

fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let command = TodoAction::guess_command(&arguments[0]);
    let other_commands = arguments.into_iter().skip(1).collect::<Vec<_>>();

    match command {
        TodoAction::List => list_item(),
        TodoAction::Add => add_item(other_commands),
        TodoAction::Done => todo!("Handle Done command"),
        TodoAction::Remove => todo!("Handle Remove command"),
        TodoAction::Unknown => println!("Invalid command provided!"),
    }

    println!("{}", style("🚀 Done!").green());
}
