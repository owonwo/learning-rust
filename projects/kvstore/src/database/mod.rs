use std::collections::HashMap;

pub struct Database {
    file: String,
    inner: std::collections::HashMap<String, String>,
}

impl Database {
    pub fn new(file_path: &str) -> Result<Database, std::io::Error> {
        let file = std::fs::read_to_string(file_path)?;
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
            Ok(_) => println!("Success Written"),
            Err(err) => println!("Error Failed: {}", err)
        }

        return Some(());
    }
}
