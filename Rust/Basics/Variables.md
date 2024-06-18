1. Variables are immutable by default. Definition can be omitted and can be done later.
2. ==mut== lets variable to be changed
3. Constants must have a type defined

```rust
fn main() {
    // Definition
    let x = 10;
    println!("x is {x}");

    // Mutability
    let mut y = 5;
    y = 10;

    // Shadowing
    let t = 10;
    let t = t + 15;

    let z = 10;

    // Constants
    const MAX: u32 = 100;
    println!("{}", z);
}
```

## Compound Data Types