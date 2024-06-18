//
//
//
//  Box Pointer
//
//
//
struct HugeData;
struct SmallData;

trait Storage {}
impl Storage for HugeData {}
impl Storage for SmallData {}
fn main() {
    let data_1 = HugeData;
    let data_2 = Box::new(HugeData);

    let data_3 = data_1; // entire data is copied, it's on stack
    let data_4 = data_2; // only the pointer itself is copied that is on stack,
                         // not the data on heap.

    let data_5 = Box::new(SmallData);

    /*
    Error in adding data_5 to the vector:
    mismatched types expected struct `Box<HugeData>`
    found struct `Box<SmallData>`

    Solution: Use trait objects to tell the compiler that this vector has items that implement
    the Storage trait.
    */
    // let data = vec![Box::new(data_3), data_4, data_5];

    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5];
}
