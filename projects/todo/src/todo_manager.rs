use crate::todo::TodoItem;

pub trait TodoManager  {
    fn get_items(&self) -> Result<Vec<TodoItem>, anyhow::Error>;

    fn add_item(&self, item: TodoItem) -> Result<(), anyhow::Error>;
    
    fn check_item(&self, item_index: &String) -> Result<(), anyhow::Error>;
}
