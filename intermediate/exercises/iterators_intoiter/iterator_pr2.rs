// Problem 2: Complete the into_iter function definition

struct Person {
    name: String,
    age: u32,
    occupation: String,
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        /* Your code here
        Hint: Should return a vector of Strings,
        representing the fields of the struct  */
        vec![self.name, self.age.to_string(), self.occupation].into_iter()
    }
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        occupation: "Software Engineer".to_string(),
    };

    let mut person_iterator = person.into_iter();

    // for property in person_iterator {
    //     println!("{property}");
    // }

    while let Some(property) = person_iterator.next() {
        println!("{}", property);
    }
}
