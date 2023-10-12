use std::collections::HashMap;

pub struct Database {
    file: String,
    inner: std::collections::HashMap<String, String>,
}

impl Database {

    pub fn get_file(file_path: &str) -> Result<String, std::io::Error> {
        let file = std::fs::read_to_string(file_path);

        return match file {
            Ok(file) => Ok(file),
            Err(err) => {
                println!("Database file missing{}", err);
                println!("Creating a new file: {}", file_path);
                std::fs::write(file_path, "")?;
                return std::fs::read_to_string(file_path);
            }
        }
    }

    pub fn new(file_path: &str) -> Result<Database, std::io::Error> {
        let file = Database::get_file(file_path)?;

        // write a new file and then reference the file
        let mut inner = HashMap::new();

        for line in file.lines() {
            let pair = line.split("\t").collect::<Vec<_>>();
            inner.insert(pair[0].to_owned(), pair[1].to_owned());
        }

        return Ok(Database {
            file: file_path.to_owned(),
            inner,
        });
    }

    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.inner.insert(key.to_owned(), value.to_owned())
    }

    pub fn save_changes(&self) -> Option<()> {
        let mut text_content = vec![];

        for (key, value) in self.inner.iter() {
            text_content.push(format!("{}\t{}", key, value));
        }

        println!("{}", text_content.join("\n"));

        match std::fs::write(&self.file, text_content.join("\n")) {
            Ok(_) => println!("💾 Changes saved"),
            Err(err) => println!("Error Failed: {}", err),
        }

        return Some(());
    }
}

impl From<Database> for Vec<(String, String)> {
    fn from(value: Database) -> Self {
        return value
            .inner
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect::<Vec<_>>();
    }
}