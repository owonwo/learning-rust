use crate::todo::TodoItem;

pub trait TodoManager  {
    fn get_items(&self) -> Result<Vec<(String, String)>, anyhow::Error>;

    fn add_item(&self, item: TodoItem) -> Result<(), anyhow::Error>;
}
