# String Slice and String
```rust
let str = String::new();
let str2 = String::from("Test String");

let fixed_str = "Fixed length string"; // string slice
let mut flexible_str = String::from("This string will grow");
```

## Concatenate Two Strings, push_str()
```rust
let mut str1 = String::from("Hello ");
str1.push_str("World");

println!("{:?}", str1); // Hello World
```


## Various String vs &str occassions
```rust
fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
```
---

# Vector
```rust
let vec: Vec<i32> = Vec::new();
let vec = vec![1, 5, 10, 29];
```

## Useful methods
```rust
// Sum of values
let sum = vec.iter().sum();
```

---

# Tuple
```rust
let my_info = ("Salary", 40000, "Age", 40);

let salary_value = my_info.1;

let (salary, salary_value, age, age_value) = my_info;

print!("{salary}, {salary_value}, {age}, {age_value}");
```

# Unit Type
```rust
// Unit type, related to Tuple
let unit = ();
```

---
