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

## Associated Values of Variants

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

# Standard Library Enums
[[Option]]
[[Result]]