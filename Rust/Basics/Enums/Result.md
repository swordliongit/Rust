
- Use it to represent success or failure of an operation

```rust
enum Result<E, T> {
    Ok(T),
    Err(E),
}
```

```rust
fn check_grade(student_name: &String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found!"))
}
```