1. Box is similar to unique_ptr in C++

## Problem:
#### Idea is to have lists that contain other lists in the program below.

#### ==Error==: recursive type `List` has infinite size.

#### ==Explanation==: 
Problem with recursive types is that,
the Rust compiler is unable to know the exact size of the instance of such type at compile time.
Rust has to know the sizes of variables at compile time.

#### How does Rust calculate the memory size of a variable?:
For example:
    enum Conveyance {
        Car(i32),
        Train(i32),
        Air(i32),
        Walk
    }

    - Rust goes through the variants to determine how much memory to allocate.
    - Car, Train and Air require a fixed size memory that is i32 and Walk doesn't take any mem space
    - Rust allocates an i32 sized memory space for the enum. So the Conveyance enum takes i32 space
    - in the memory.

    - In contrast to the List enum below, the compiler knows Nil won't take any space.
    - But it can't determine how much space the Cons variant will take because the second value of
    - Cons is a List in itself, that is recursive in nature.
    - It's unknown how many times it will recurse.
    - Solution to this is to use the Box pointer.

```rust
#[derive(Debug)]
enum List {
    Cons(i32, List),
    Nil,
}
fn main() {

    let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
}
```

## Solution

: Compiler now knows the exact size of the second value of the Cons variant, which is the size of a Box pointer. The Box itself is on the Stack, so it's a fixed size but the data it points to is in heap, which allows for recursive data.

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("{:?}", list);
}
```

## Improved Version using Option

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}
fn main() {
    let x = 0.600;
    let y = Box::new(x); // points to a heap memory containing the value of x
    let z = &x; // stack memory pointer

    // last variant doesn't need heap allocation, so it's just None
    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
    );

    println!("{:?}", list);
}
```

# Vector of Different Types and Further Info on Box Pointer

[[Trait Objects && Dynamic Dispatch]]

```rust
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
```

# Box Examples


```rust
// Problem 1: Fix the code below so that it compiles 
// Solution: 

enum BinaryTree {
    Leaf,
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
}

fn main() {}
```


```rust
// Problem 2: Fix the code by completing the function signature
// Solution: 
struct Wrapper {
    data: String,
}

fn modify_data(mut wrapper: Box<Wrapper>) -> Box<Wrapper> {
    wrapper.data = String::from("Modified");
    wrapper
}

fn main() {
    let original_wrapper = Box::new(Wrapper {
        data: String::from("Original"),
    });
    let modified_wrapper = modify_data(original_wrapper);
}
```

```rust
// Problem 3: Complete the code below
// Solution: 

#[derive(Debug)]
enum ListNode<T> {
    Node(T, Box<ListNode<T>>),
    None,
}

fn main() {
    // Create a linked list representing: Node(1, Node(2, Node(3, Node(4, None))))
    let list = ListNode::Node(
        1,
        Box::new(ListNode::Node(
            2,
            Box::new(ListNode::Node(
                3,
                Box::new(ListNode::Node(4, Box::new(ListNode::None))),
            )),
        )),
    );
    println!("{:?}", list);
}
```

```rust
// Problem 4: Fix the code by adding the type annotation 

struct AudioSample;
struct ImageFile;

trait Media {}

impl Media for AudioSample {}
impl Media for ImageFile {}

fn main() {
    let audio_1 = AudioSample;
    let audio_2 = Box::new(AudioSample);

    let audio_3 = audio_1;
    let audio_4 = audio_2;

    let image_1 = Box::new(ImageFile);

    let media_collection: Vec<Box<dyn Media>> = vec![Box::new(audio_3), audio_4, image_1]; // Fix this line
}
```