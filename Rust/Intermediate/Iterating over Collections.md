```rust
fn main() {
    let mut vec_1 = vec![45, 30, 85, 90, 41, 39];
    
    let mut vec_1_iter: std::slice::Iter<i32> = vec_1.iter();
    let mut vec_1_iter: std::slice::IterMut<i32> = vec_1.iter_mut();
    let mut vec_1_iter: std::vec::IntoIter<i32> = vec_1.into_iter();
    
    let value_1 = vec_1_iter.next();
    
    for values in vec_1 {
        println!("{values}");
    }
}
```