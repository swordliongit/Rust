1. String slices have inherent &'static lifetime

# 1. ==Code blocks== can be assigned to variables and they can return values
```rust
let name = {
	let fname = "Sword";
	let lname = "Lion";
	format!("{fname} {lname}")
};
```

# 2. Printing Compound types:
```rust
let mut array_1 = [4, 5, 6, 7, 8];
println!("{:?}", array_1);
```

# 3.Input:
```rust
let mut n = String::new();

std::io::stdin()
	.read_line(&mut n)
	.expect("Failed to read input!");

let n: i64 = n.trim().parse().expect("Invalid Input!");
println!("{n}");
```

# 4. Println Arguments
```rust
// positional arguments
let n1 = 10;
let n2 = 15;
println!("{2} + {1} equals to {0}", n1 + n2, n1, n2);

// named arguments
println!(
	"{game} is one of the best RPGs ever made",
	game = "No Rest for the Wicked"
);
```

# 5. Conventions
```rust
let x = 40_000_000;
println!("{x}");
```

# 6. Static and Const
```rust
static MSG: &str = "Welcome home my child"; // All references to a static refers to the same memory location, 1 instance of the value
const PI: f32 = 3.14; // constants are inlined and do not occupy a specific location in memory

let a = PI;
let b = PI;

let c = MSG;
let d = MSG;
```

# 7. System Time
```rust
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
```

# Useful Functions

## Char
- is_alphabetic()
- is_numeric()
- as_str() : Chars iterator -> &str
```rust
let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
```

## String and &str
- ==repeat()==
```rust
fn main() {
    let hello = String::from("hello");
    let world_repeated = &"world".repeat(5); // & is appended to the output of repeat(), not the "world"
    let hello_world = hello + world_repeated;

    println!("{hello_world}"); // helloworldworldworldworldworld
}
```
- ==to_owned()==
- ==to_string()== : &str -> String
- ==chars()==: Returns an iterator over the chars of a string slice.
- ==last()==: Consumes the iterator, returning the last element.
```rust
let str = "Test string";
let ch = str.chars().last().unwrap();
```