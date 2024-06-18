//
//
//
//  Lifetimes in Structs
//
//
//

/*
1. It's quite possible that the reference becomes invalid while
the instance of the Struct is still alive.
2. Struct fields that are references to data, must have explicit lifetime specifiers.
3. Lifetime Elisions are not defined for Structs.
*/

// The field this struct is pointing must have a lifetime as long as the struct instance has.
struct ArrayProcessor<'a> {
    data: &'a [i32],
}

/*
implicit elided lifetime not allowed here
expected lifetime parameter
*/
impl<'a> ArrayProcessor<'a> {
    fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] {
        // rule 3, lifetime of self is assigned to return value, so no explicit lifetime needed
        let old_data = self.data;
        self.data = new_data; // explicit lifetime required in the type of `new_data`
        &old_data
    }
}
// Expanded to this:
// impl<'a> ArrayProcessor<'a> {
//     fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &'b [i32] {
//         // rule 3, lifetime of self is assigned to return value, so no explicit lifetime needed
//         let old_data = self.data;
//         self.data = new_data; // explicit lifetime required in the type of `new_data`
//         &old_data
//     }
// }

fn main() {
    let mut some_data = ArrayProcessor { data: &[2, 1, 100] };

    let prev_data = some_data.update_data(&[1, 100, 1000]);
    println!("Previous data : {:?}", prev_data);
    println!("New data: {:?}", some_data.data);
}
