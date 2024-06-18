- Multiple trait bounds can be added using +

#### Generics can be bounded by traits. T is a type that implements the Shape trait, T is bounded by Shape:
```rust
// T is a type that implements the Shape trait, T is bounded by Shape
fn shape_properties<T: Shape>(object: T) {
    object.area();
    object.perimeter();
}

// Same meaning:
fn shape_properties<T>(object: impl Shape) {
    object.area();
    object.perimeter();
}

// Same meaning:
fn shape_properties<T>(object: T)
where
    T: Shape,
{
    object.area();
    object.perimeter();
}


struct Circle {
    radius: f32,
}

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

    shape_properties(rect);
    shape_properties(square);

	let circle = Circle { radius: 10.0 };
	
    shape_properties(circle); // the trait bound `Circle: Shape` is not satisfied
}```

## Returning Bound Traits

```rust
// Returns a type that implements Shape trait
fn returns_shape() -> impl Shape {
    let sq = Square {
        side: 5.0,
        line_width: 5,
        color: String::from("Red"),
    };
    sq
}
```

#### Only 1 single type which implements the trait can be returned:
```rust
// Returns an object that implements Shape trait
fn returns_shape() -> impl Shape {
    let sq = Square {
        side: 5.0,
        line_width: 5,
        color: String::from("Red"),
    };
    // sq
    let rect = Rectangle {
        length: 5.0,
        width: 10.0,
        line_width: 5,
        color: String::from("Green"),
    };
    let chck = false;
    if chck {
        sq
    } else {
        rect // ERROR
    }
}
```

## Alternative impl trait Function Parameters

#### Parameters accept any type that implements VehicleHorn:
```rust
pub trait VehicleHorn {
    fn horn_sound(&self) -> String {
        "peep peep".to_string()
    }
}

struct Car {}

struct Truck {}

impl VehicleHorn for Car {}
impl VehicleHorn for Truck {}

fn compare_horn_sound(vehicle_1: impl VehicleHorn, vehicle_2: impl VehicleHorn) -> bool {
    vehicle_1.horn_sound() == vehicle_2.horn_sound()
}

fn main() {
    let car = Car {};
    let truck = Truck {};
    assert_eq!(compare_horn_sound(car, truck), true);
}
```

## Multiple Trait Bounds

```rust
trait SquareRoot {
    fn square_root(&self) -> Self;
}

trait Displayable {
    fn to_display_string(&self) -> String;
}

fn get_square_root_str(input: impl SquareRoot + Displayable) -> String { // make changes to this line only
    let squared_rooted = input.square_root();
    squared_rooted.to_display_string()
}

impl SquareRoot for f64 {
    fn square_root(&self) -> Self {
        self.sqrt()
    }
}

impl Displayable for f64 {
    fn to_display_string(&self) -> String {
        format!("{:.2}", self)
    }
}

fn main() {
    let num = 9.0;
    let mut msg = format!("{num} square rooted is ");
    msg.push_str(&get_square_root_str(num));
    println!("{msg}");
}
```
