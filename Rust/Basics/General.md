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


# Useful Functions

## Char
- is_alphabetic()
- is_numeric()
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