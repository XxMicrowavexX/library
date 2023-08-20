use std::fmt::{self, Display};

pub struct Book {
    pub name: String,
    pub author: String,
    pub content: Vec<String>,
}

impl Book {
    #[must_use]
    pub fn new(name: &str, author: &str) -> Self {
        Self {
            name: name.to_string(),
            author: author.to_string(),
            content: vec![],
        }
    }
}
impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, page) in self.content.iter().enumerate() {
            write!(f, "{page}\n\n{i}\n\n\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut book = Book::new("Welcome to lib", "Microwave");

        book.content.push("Welcome".to_string());

        println!("{book}");
    }
}
