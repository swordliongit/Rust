
- In Rust preload, so it's auto-loaded in every program
- it's used in place of null in other languages
- Use if you want to represent presence or absence of value
#### Useful methods
---
1. is_some()
2. unwrap() : Returns the contained `Some` value, consuming the `self` value.
3. flatten()  : Clears None variants in a vector of Options
---


```rust
// enum Option<T> {
//     None,
//     Some(T),
// }
```

```rust
struct Student {
    name: String,
    grade: Option<u32>,
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Sword"),
            grade: Some(78),
        },
        Student {
            name: String::from("Lion"),
            grade: Some(90),
        },
        Student {
            name: String::from("Contra"),
            grade: None,
        },
    ];
}
```


# Destructuring Option Variants - if let
- Good if we care about only 1 variant and discard all others

```rust
let student_grade = get_grade(&student_name, &student_db); // can return either None or Some()

match student_grade {
	Some(grade) => println!("{}'s grade is {}", student_name, grade),
	None => {}
}

// identical but cleaner code
if let Some(grade) = student_grade {
	println!("{}'s grade is {}", student_name, grade);
}
```

---

# Combinators with Option

```rust
fn main() {
	let some_product = Some("laptop");
	let mut products = vec!["cellphone", "battery", "charger"];
	
	match some_product {
		Some(product) => products.push(product),
		_ => {}
	};
	
	if let Some(product) = some_product {
		products.push(product);
	}
	
}
```

## Extend
- Achieves the same result as the above code. Adds the destructured Option value into the vector

```rust
products.extend(some_product);
```


---

# Filtering None in a Vector of Options

```rust
let products = vec![Some("Laptop"), Some("Cable"), Some("Keyboard"), None];
```

## First option:
```rust
let mut filtered_products = Vec::new();

for product in products {
	if product.is_some() {
		filtered_products.push(product.unwrap());
	}
}
```

## Second option:
```rust
let filtered_products = products
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<&str>>();
```

## Third, the Best option:
```rust
let filtered_products: Vec<&str> = products.into_iter().flatten().collect();
println!("{:?}", filtered_products);
```

---

# Nested Options

```rust
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
```
```