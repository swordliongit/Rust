- Trait with no body. Acts as a marker to which traits are required, groups them together. Super traits are used.

```rust
trait Properties: PartialEq + Default + Clone {} // Marker trait

#[derive(Debug, PartialEq, Default, Clone)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}

impl Properties for Student {}

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

    println!("Student: {:?}", s1);
    println!("s1 and s2 are equal: {}", s1 == s2);
}
```