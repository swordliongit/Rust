- Anonymous functions that can capture variables from outer scopes
- 3 types of traits that get implemented for the closure:
	1. Immutable borrow : Fn
	2. Mutable borrow : FnMut
	3. Change of ownership : FnOnce

#### 1. impl Fn(&str) -> bool
```rust
let banned_user = String::from("Banned User");

let validate_user_simple = |name: &str| {
	let banned_user_inner = &banned_user;
	name.len() != 0 && name != banned_user_inner
};
```

#### 2. impl FnMut(&str) -> bool
```rust
let mut banned_user = String::from("Banned User");

let validate_user_simple = |name: &str| {
	let banned_user_inner = &mut banned_user;
	name.len() != 0 && name != banned_user_inner
};
```

#### 3. impl FnOnce(&str) -> bool
```rust
let banned_user = String::from("Banned User");
 
let validate_user_simple = |name: &str| {
	let banned_user_inner = banned_user;
	name.len() != 0 && name != banned_user_inner
};
```


# Moving all the captured values - ==move== keyword
- `move` converts any variables captured by reference or mutable reference to variables captured by value.

```rust
let banned_user = String::from("Banned User");
 
let validate_user_simple = move |name: &str| {
	let banned_user_inner = banned_user;
	name.len() != 0 && name != banned_user_inner
};
```

# Function Pointer
- function pointers implement all the 3 closure traits
- functions can be passed to anywhere where a closure is expected

```rust
fn wrapper_func(text: &str, fp: fn(&str) -> usize) {
    fp(text);
}

fn text_length(text: &str) -> usize {
    text.len()
}

fn main() {
	wrapper_func("example", text_length);
}
```