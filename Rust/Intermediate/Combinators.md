1. Combinators methods on iterators that are compact pure functions, can be linked together to execute complex operations.
2. They are lazy methods, no work is done until their next() is called.

---
1. ==into_iter()==: Creates a consuming iterator, that is, one that moves each value out of the vector (from start to end). The vector cannot be used after calling this.
2. ==filter()==: Creates an iterator which uses a closure to determine if an element should be yielded. Given an element the closure must return true or false. The returned iterator will yield only the elements for which the closure returns true.
3. ==map()==: Takes a closure and creates an iterator which calls that closure on each element. `map()` transforms one iterator into another, by means of its argument: something that implements `FnMut`. It produces a new iterator which calls this closure on each element of the original iterator.
4. ==collect()==: Transforms an iterator into a collection.
5. ==sum()==: Sums the elements of an iterator. Takes each element, adds them together, and returns the result.
---



#### Instead of the below code, we can make it more clear using combinator methods:
```rust
fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    let mut result: Vec<String> = vec![];

    for word in words {
        if word.starts_with("a") || word.starts_with("b") {
            let uppercase_word = word.to_uppercase();
            result.push(uppercase_word);
        }
    }
    println!("{:?}", result);
}
```

#### Various combinator methods:
```rust
fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    
    let result: Vec<String> = words
        .into_iter()
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        .map(|word| word.to_uppercase())
        .collect(); // collect::<Vec<String>> turbo fish syntax as alternative

    println!("{:?}", result);
}
```

---


