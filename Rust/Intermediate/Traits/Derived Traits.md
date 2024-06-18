```rust
#[derive(Debug, PartialEq)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}

fn main() {
    let s1 = Student {
        name: String::from("Mark"),
        age: 35,
        sex: 'M',
    };
    let s2 = Student {
        name: String::from("Jessica"),
        age: 24,
        sex: 'F',
    };

    println!("Student: {:?}", s1); // Debug trait required for printing
    println!("s1 and s2 are equal: {}", s1 == s2); // PartialEq required for comparison
}
```