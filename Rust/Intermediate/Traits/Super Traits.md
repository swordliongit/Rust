1. All items implementing the base trait, have to implement the Super trait

```rust

// super trait
trait Draw {
    fn draw_object(&self);
}

trait Shape: Draw {
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
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Drawing Rectangle");
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("Drawing Square");
    }
}
```

## Using Super Traits to cut down on extra code

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

// traits
trait OtherTrait {} // marker trait, empty
impl OtherTrait for Rectangle {}
impl OtherTrait for Square {}

trait SomeOtherTrait {}
impl SomeOtherTrait for Rectangle {}
impl SomeOtherTrait for Square {}

trait Shape {
    fn area(&self) -> f32;

    // Default implementation
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented. Returning empty value.");
        0.0
    }
}

// Bounds have to be specified 
fn shape_properties<T>(object: T)
where
    T: Shape + OtherTrait + SomeOtherTrait,
{
    object.area();
    object.perimeter();
}

```

#### Solution:

```rust
trait Shape: OtherTrait + SomeOtherTrait {
    fn area(&self) -> f32;

    // Default implementation
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented. Returning empty value.");
        0.0
    }
}


// No need to specify extra bounds, Shape already has them as superset
fn shape_properties<T>(object: T)
where
    T: Shape,
{
    object.area();
    object.perimeter();
}
```
