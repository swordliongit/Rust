1. ==Monomorphization== is used to generate only the used versions of the function implementations to make generics zero cost. Also called ==Static Dispatch==
2. Monomorphization happens in ==compile time==. So there's no run time cost.
3. ==1 concern== with monomorph is the ==code bloat==. If you use a generic function with many different types, it might generate multiple copies of the function. Solved with ==Dynamic Dispatch==
4. ==Dynamic Dispatch==: Specific implementations will not be generated at compile time.

# General Example:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

fn main() {
	let origin = Point::new(0, 0);
	let p1 = Point { x: 1.2, y: 3.5 };
}
```

---

# Specialization
- Can't have duplicate named methods unless they are a specialization

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

// Specialization
impl Point<i32, i32> {
    fn print_coord(&self) {
        println!("Point: {}, {}", self.x, self.y);
    }
    
	fn new() { // ERROR
		...
	}
}

impl Point<f64, f64> {
    // Duplicate name is allowed for different specializations
    fn print_coord(&self) {
        println!("Point: {}, {}", self.x, self.y);
    }
}

fn main() {
    let origin = Point::new(0, 0);
    let p1 = Point { x: 1.2, y: 3.5 };

    let p2 = Point { x: 12.0, y: 7 };

    origin.print_coord();
    p1.print_coord();
    
    p2.print_coord(); // ERROR i32, f64
}
```

---

# Free Functions

```rust
// Free Functions
fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
    unimplemented!();
}

fn main() {
    let origin = Point::new(0, 0);
    let p1 = Point { x: 1.2, y: 3.5 };

    add_points(&origin, &origin);
    add_points(&p1, &p1);
}
```

## Monomorphization Example

```rust
add_points(&origin, &origin);
add_points(&p1, &p1);

// Compiler Creates
add_points_i32(&origin, &origin);
add_points_f64(&p1, &p1);

///////////////////
// Concrete implementations
fn add_points_i32<i32, i32>(p1: &Point<i32, i32>, p2: &Point<i32, i32>) -> Point<i32, i32> {
    unimplemented!();
}

fn add_points_f64<f64, f64>(p1: &Point<f64, f64>, p2: &Point<f64, f64>) -> Point<f64, f64> {
    unimplemented!();
}
```