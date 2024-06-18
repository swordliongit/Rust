1. Trait is the way to achieve polymorphism in Rust
2. Trait is an interface that can be used by data types to have shared functionality

#### Consider this example. We don't have any way of restricting the area function, we can end up with widly different results on the same idea:

```rust
struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

// Different implementations of the same idea
impl Square {
    fn calculate_area(&self) {
        println!("The area is: {}", self.side * self.side);
    }
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
}
```


#### Solution with Trait:

```rust
trait Shape {
    fn area(&self) -> f32;
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("Rectangle area: {}", area_of_rect);
        area_of_rect
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }
}
```

---


# Default Implementation

```rust
trait Shape {
    fn area(&self) -> f32;

    // Default implementation
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented. Returning empty value.");
        0.0
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("Rectangle area: {}", area_of_rect);
        area_of_rect
    }

    // Overrides the default impl
    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("Perimeter of rect: {}", perimeter_of_rect);
        perimeter_of_rect
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }

	// No impl for perimeter, will resort to the default one
}
```

```rust
fn main() {
    let rect = Rectangle {
        length: 2.3,
        width: 5.0,
        line_width: 1,
        color: String::from("Blue"),
    };

    let square = Square {
        side: 4.1,
        line_width: 1,
        color: String::from("Turqoise"),
    };

    rect.area();
    square.area();
    rect.perimeter();
    square.perimeter();
}
```

## When there's only 1 trait method

```rust
struct Car {
    model: String,
    speed: f64,
}

struct Bicycle {
    brand: String,
}

struct Bus {
    model: String,
    speed: f64,
}

trait Vehicle {
    fn speed(&self) -> f64 {
        0.0
    }
}

impl Vehicle for Car {
    fn speed(&self) -> f64 {
        self.speed
    }
}

impl Vehicle for Bicycle {} // Empty trait impl

impl Vehicle for Bus {
    fn speed(&self) -> f64 {
        self.speed
    }
}
```


---

# ==Self== in Traits refer to the type that implements the Trait

```rust
// Problem 2: Try identifying the error in the code
// Hint: The error is related to the concept of supertrait

trait Size {
    fn compute_size(&self) -> u16;
}

trait Printable {
    fn size_to_str(&self) -> String;
}

trait Comparable: Size + Printable {
    fn print_greater(a: &Self, b: &Self) { 
    // Please note that Self on the line above means, that type which will be implementing the trait 

        let item1 = a.compute_size();
        let item2 = b.compute_size();
        if item1 > item2 {
            println!("{} is greater than {}", a.size_to_str(), b.size_to_str());
        } else if item2 > item1 {
            println!("{} is greater than {}", b.size_to_str(), a.size_to_str());
        } else {
            println!("Both sizes are {}", a.size_to_str());
        }
    }
}

struct Book {
    page: u16,
}

impl Size for Book {
    fn compute_size(&self) -> u16 {
        self.page
    }
}

impl Printable for Book {
    fn size_to_str(&self) -> String {
        format!("Book having {} pages", self.page)
    }
}

impl Comparable for Book {}

fn main() {
    let book_1 = Book { page: 50 };
    let book_2 = Book { page: 450 };
    Comparable::print_greater(&book_1, &book_2);
}

```

# Trait Bounds
[[Trait Bounds]]
# Super Traits
[[Super Traits]]
# Trait Object and Dynamic Dispatch
 [[Trait Objects && Dynamic Dispatch]]
# Derived Traits
[[Derived Traits]]
# Associated Traits
[[Associated Traits - Generic Traits]]