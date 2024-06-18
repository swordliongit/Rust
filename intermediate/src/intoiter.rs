//
//
//
//  IntoIterator
//
//
//

// trait IntoIterator {
//     type Item;
//     type IntoIter: Iterator;
//     fn into_iter(self) -> Self::IntoIter;
// }

struct Book {
    title: String,
    author: String,
    genre: String,
}

struct BookIterator {
    properties: Vec<String>,
}

// impl Iterator for BookIterator {
//     type Item = String;
//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.properties.is_empty() {
//             Some(self.properties.remove(0))
//         } else {
//             None
//         }
//     }
// }

// impl IntoIterator for Book {
//     type Item = String;
//     type IntoIter = BookIterator;

//     // consumes self
//     fn into_iter(self) -> Self::IntoIter {
//         BookIterator {
//             properties: vec![self.title, self.author, self.genre],
//         }
//     }
// }

// Alternative, converting to a vector iterator
impl IntoIterator for Book {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    // consumes self
    fn into_iter(self) -> Self::IntoIter {
        vec![self.title, self.author, self.genre].into_iter()
    }
}

fn main() {
    let book = Book {
        title: "Digital Image Processing".to_string(),
        author: "Gonzales".to_string(),
        genre: "Science Book".to_string(),
    };

    // consumes the Book instance, essentially convertes the book into a BookIterator
    let mut book_iterator = book.into_iter();

    for book_info in book_iterator {
        println!("{book_info}");
    }
}
