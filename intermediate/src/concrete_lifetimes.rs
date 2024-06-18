//
//
//
// Concrete Lifetimes
//
//

// Non lexical lifetime : Lifetime that is not tied to scopes

fn main() {

    // immutable and mutable references co-exist here because they don't overlap their scope lifetimes
    let mut vec_1 = vec![6, 5, 8, 9];
    let ref_1 = &vec_1;
    println!("ref 1: {:?}", ref_1); // Last usage of ref_1
    let ref_2 = &mut vec_1;
    ref_2.push(100);
    println!("ref 2: {:?}", ref_2); // Last usage of ref_2
}
