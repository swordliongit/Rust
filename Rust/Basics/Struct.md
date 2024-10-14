1. ==self== is the same as ==self: Self==
2. Values of a struct are called ==Fields==

```rust
struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}
```

# Other Struct Types

```rust
// tuple struct

struct Point_2D(i32, i32);
struct Point_3D(f64, f64, f64);

let p2D = Point_2D(5, 25);
let p3D = Point_3D(10.0, 20.0, 30.0);

let (x, y, z) = (p3D.0, p3D.1, p3D.2);

// unit struct
struct UnitStruct;
let message = format!("{:?}s are fun!", UnitStruct);
// UnitStructs are fun!
```

---

# Update Syntax ..

```rust
#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        // TODO: Create your own order using the update syntax and template above!
        let your_order = Order {
            name: "Hacker in Rust".to_string(),
            count: 1,
            ..order_template
        };
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
```

# Partial Move

```rust
fn main() {
    let car_1 = Car {
        owner: String::from("Sword"),
        year: 2024,
        fuel_level: 55.0,
        price: 1_500_000,
    };

    let moved_owner = car_1.owner; // value pointed to is in heap and other parts in stack
    // car_1.owner doesn't exist here
    println!("{}", car_1.owner); // borrow of moved value, ERROR
    
	let copied_owner = car_1.owner.clone();
	println!("{}", car_1.owner);  // works
}
```


```rust
fn main() {
	let car_1 = Car {
		owner: String::from("Sword"),
		year: 2024,
		fuel_level: 55.0,
		price: 1_500_000,
	};
	
	let car_2 = Car {
		owner: String::from("Lion"),
		..car_1 // car_1 fields are COPIED except the owner
	};

	let car_3 = Car {
        ..car_1 // owner is MOVED to car_3 and other fields are COPIED as they are in stack
    };
    println!("{}", car_1.owner); // borrow of moved value ERROR

}
```

---

# Passing and returning structs

```rust
struct Fruit {
    apples: i32,
    bananas: i32,
}

fn increase_fruit(fruit: Fruit) -> Fruit {
    Fruit {
        apples: fruit.apples * 2,
        bananas: fruit.bananas * 3,
    }
}

fn new_fruit() -> Fruit {
    Fruit {
        apples: 10,
        bananas: 5,
    }
}

fn print_fruit(fruit: Fruit) {
    println!(
        "You have {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
}

fn main() {
    // let some_fruit = new_fruit();
    // let updated_fruit = increase_fruit(new_fruit());
    print_fruit(increase_fruit(new_fruit()));
}

```

---

# Functionality in Structs, methods - impl 

```rust
struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    fn display_info(&self) {
        println!(
            "Owner: {}, Year: {}, Price: {}",
            self.owner, self.year, self.price
        );
    }
}

fn main() {
    let mut car_1 = Car {
        owner: String::from("Sword"),
        year: 2024,
        fuel_level: 55.0,
        price: 1_500_000,
    };

    car_1.display_info();
}
```

---

# Associated Method ( Static Function )

```rust
struct Car {
    owner: String,
    year: u32,
    fuel_level: f64,
    price: u32,
}

impl Car {
    fn monthly_insurance() -> u32 {
        200
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }
}
```

---
