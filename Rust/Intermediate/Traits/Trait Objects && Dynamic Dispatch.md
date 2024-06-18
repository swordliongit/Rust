1. Trait objects allow us to define a type which implements a trait without knowing the type itself.
2. Specialized versions of the function will not be generated.
3. Resolution of the function will take place in run-time.
4. Trait object needs to be behind a pointer.
5. Solves the conditional generic impl return problem. // insert link here

#### Example 1: 
```rust
fn shape_properties_static<T>(object: T)
where
    T: Shape + OtherTrait + SomeOtherTrait,
{
    object.area();
    object.perimeter();
}

fn shape_properties_dynamic(object: Box<dyn Shape>) {
    object.area();
    object.perimeter();
}

fn main() {
	....
	shape_properties_dynamic(Box::new(rect));
	shape_properties_dynamic(Box::new(square));
}
```

#### Example 2:
```rust
trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
}

struct Lion {
    name: String,
}

impl Animal for Lion {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("Roar!");
    }
}

struct Penguin {
    name: String,
}

impl Animal for Penguin {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("Honk!");
    }
}

fn describe_animal(animal: &dyn Animal) {
    // This line needs a fix
    println!("The {} says:", animal.name());
    animal.make_sound();
}

fn main() {
    let lion = Lion {
        name: "Simba".to_string(),
    };
    let penguin = Penguin {
        name: "Happy Feet".to_string(),
    };

    // The calls to function needs fixes
    describe_animal(&lion);
    describe_animal(&penguin);
}

```

## Returning Dynamic Traits

#### Problem:
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

#### Solution:
```rust
fn returns_shape_dynamic(dimension: Vec<f32>) -> Box<dyn Shape> {
    if dimension.len() == 1 {
        let sq = Square {
            side: 5.0,
            line_width: 5,
            color: String::from("Red"),
        };
        Box::new(sq)
    } else {
        let rect = Rectangle {
            length: 5.0,
            width: 10.0,
            line_width: 5,
            color: String::from("Green"),
        };
        Box::new(rect)
    }
}
```