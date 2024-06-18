//
//
//
//  Iterator
//
//
//

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
struct Employee {
    name: String,
    salary: u16,
}

struct Employee_Records {
    employee_db: Vec<Employee>,
}

impl Iterator for Employee_Records {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let mut emp_1 = Employee {
        name: String::from("John"),
        salary: 40_000,
    };
    let mut emp_2 = Employee {
        name: String::from("Josh"),
        salary: 30_000,
    };

    let mut emp_db = Employee_Records {
        employee_db: vec![emp_1, emp_2],
    };

    for emp in emp_db {
        println!("{emp}");
    }
}
