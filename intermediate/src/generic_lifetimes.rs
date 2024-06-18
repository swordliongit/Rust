//
//
//
//  Generic Lifetimes
//
//
//

fn main() {
    // Compiler doesn't know if i or j will be returned and it doesn't know
    // if the references past will live enough for the returned value to be used
    // After specifying explicit lifetimes, compiler now knows that the references
    // has to live as long as the return value, so it doesn't let the int2 in this case
    // to go out of scope, because it'd be destroyed before the return value would be used,
    // that is the "picked_value". Causing a dangling reference if a reference was returned
    // to int2

    // Lifetime of picked_value is equal to the shortest living reference, that is int2
    let int1 = 5;
    let picked_value: &i32;
    {
        let int2 = 10;
        picked_value = pick_int(&int1, &int2); // Error int2 doesn't live long enough
    }
    println!("{}", picked_value);

    // fn pick_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    //     if rand::random() {
    //         i
    //     } else {
    //         j
    //     }
    // }

    // Now the lifetime of returned value to picked_value equals to int1, that lives until the end of main
    // therefore this is valid
    // fn pick_int<'a>(i: &'a i32, j: &i32) -> &'a i32 {
    //     i
    // }

    fn pick_int(i: &i32, j: &i32) -> &'static i32 {
        let y: &'static i32 = &10; // static lifetime is the entire duration of the program
        y
    }
}
