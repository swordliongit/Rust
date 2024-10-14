1. Enum values are called ==variants==


```rust
enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn main() {
    let day = Days::Saturday;
}
```

# Associated Values of Variants
```rust
enum TravelType {
    Car(f32),
    Train(f32),
    Plane(f32),
}

impl TravelType {
    fn travel_compensation(&self) -> f32 {
        match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Plane(miles) => miles * 5.0,
        }
    }
}

fn main() {
    let car = TravelType::Car(100.0);
    println!(
        "Compensation for the customer: {}",
        car.travel_compensation()
    );
}
```

# Printing Enums
```rust
#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
```


# Struct Variant
```rust
#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: u32, y: u32 },
    Echo(String),
    ChangeColor(u32, u32, u32),
    Quit,
}

fn main() {
	let messages = [
		Message::Move { x: 10, y: 30 },
		Message::Echo(String::from("hello world")),
		Message::ChangeColor(200, 255, 255),
		Message::Quit,
	];
}
```


# Standard Library Enums
[[Option]]
[[Result]]