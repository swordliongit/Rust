- Consumes the instance called upon and returns an iterator, basically converting a type instance into an iterator

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```


```rust
struct Book {
    title: String,
    author: String,
    genre: String,
}

struct BookIterator {
    properties: Vec<String>,
}

impl Iterator for BookIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.properties.is_empty() {
            Some(self.properties.remove(0))
        } else {
            None
        }
    }
}

impl IntoIterator for Book {
    type Item = String;
    type IntoIter = BookIterator;

    // consumes self
    fn into_iter(self) -> Self::IntoIter {
        BookIterator {
            properties: vec![self.title, self.author, self.genre],
        }
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
```

# Alternative, returning a vector iterator

```rust
struct Book {
    title: String,
    author: String,
    genre: String,
}

struct BookIterator {
    properties: Vec<String>,
}


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
```

# While Let

```rust
struct Person {
    name: String,
    age: u32,
    occupation: String,
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.name, self.age.to_string(), self.occupation].into_iter()
    }
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        occupation: "Software Engineer".to_string(),
    };

    let mut person_iterator = person.into_iter();

    while let Some(property) = person_iterator.next() {
        println!("{}", property);
    }
}
```

```
Alice
30
Software Engineer
```

---
