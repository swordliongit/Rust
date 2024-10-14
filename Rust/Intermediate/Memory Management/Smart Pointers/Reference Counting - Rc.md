- Let's us have multiple owners of values
- Similar to shared_ptr in C++

# Problem:
	We want to have a couple of lists encompassing list a.

![[Screenshot 2024-06-24 164729.png]]

```rust
enum List {
    Cons(i32, Option<Box<List>>),
    Nil,
}

fn main() {
    let a = List::Cons(1, Some(Box::new(List::Cons(2, None))));
    let b = List::Cons(3, Some(Box::new(a)));
    let c = List::Cons(4, Some(Box::new(a))); // Error: use of moved value: `a`
}
```

# Solution:
```rust
use std::rc::Rc;

enum List {
    Cons(i32, Option<Rc<List>>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None))))); // first reference to this heap location
    let b = List::Cons(3, Some(Rc::clone(&a))); // a is borrowed, this clone is not a deep copy,
                                                // it increments the reference counts of a, now it's 2
    let c = List::Cons(4, Some(Rc::clone(&a))); // a is borrowed again, count is 3
} // all 3 variables will be dropped here
  // c dropped, ref count is 2
  // b dropped, ref count is 1
  // a dropped, ref count is 0
```

## Printing the reference Counts
- Rc::strong_count(&a)

```rust
fn main() {
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None))))); // first reference to this heap location
    println!("Reference count after a: {}", Rc::strong_count(&a));
    {
        let b = List::Cons(3, Some(Rc::clone(&a))); // a is borrowed, this clone is not a deep copy,
                                                    // it increments the reference counts of a, now it's 2
        println!("Reference count after b: {}", Rc::strong_count(&a));
        let c = List::Cons(4, Some(Rc::clone(&a))); // a is borrowed again, count is 3
        println!("Reference count after c: {}", Rc::strong_count(&a));
    }
    println!(
        "Reference count after inner scope: {}",
        Rc::strong_count(&a)
    );
} 
```

```
Reference count after a: 1
Reference count after b: 2
Reference count after c: 3
Reference count after inner scope: 1
```

---
