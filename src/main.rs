mod content;
mod library;

use library::Book;
use content::Content;

fn write(text: String, mut book: Book) -> Book {
    book.content.pages.push(text);

    return book;
}

fn main() {
    let harry_potter_draft = Book {
        name: String::from("Harry Potter Draft"),
        author: String::from("J.K. Rowling"),
        content: Content{ pages: Vec::new() }
    };

    println!("{}", write(String::from("Blah"), harry_potter_draft).content.pages[0]);

}
