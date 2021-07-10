use std::error::Error;

mod employee;
mod ided;
mod student;

use employee::{Employee, EmployeeRecords};
use ided::{use_idea, Ided};
use student::Student;

#[derive(Debug)]
struct MiniVec<T, U> {
    contents: Vec<T>,
    id: Option<U>,
}

impl<T, U> MiniVec<T, U> {
    fn new() -> MiniVec<T, U> {
        MiniVec {
            contents: Vec::new(),
            id: None,
        }
    }
    fn push(&mut self, x: T) {
        self.contents.push(x)
    }

    fn set_id(&mut self, id: U) {
        self.id = Some(id);
    }
}

fn use_minivec<'a, T, U>(v: &'a MiniVec<T, U>) -> &'a Vec<T> {
    &v.contents
}

fn use_ided(x: impl Ided) {
    println!("id: {}", x.my_id());
}

fn use_ided1<T: Ided>(x: T) {
    println!("id: {}", x.my_id());
}

fn use_ided2(x: Box<dyn Ided>) {
    println!("id: {}", x.my_id());
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let employee: Employee = Employee::new("SomeName".to_string(), 12);
    let student: Student = Student { student_id: 200 };
    println!("Hello {}", employee.name());
    use_idea(Box::new(employee));
    use_idea(Box::new(student));

    let empl = Employee {
        name: String::from("aaa"),
        id: 1,
    };

    let empl2 = Employee {
        name: String::from("bbb"),
        id: 2,
    };

    let mut employee_records = EmployeeRecords::new();
    employee_records.push(empl);
    EmployeeRecords::push(&mut &mut employee_records, empl2);

    println!("records {:#?}", employee_records);

    for employee in employee_records {
        println!("Employee {:?}", employee);
    }

    let mut v = MiniVec::new();
    v.push(10);
    v.set_id('5');

    let mut v2 = MiniVec::new();
    v2.push("cs");
    v2.set_id("333");

    let _v3: MiniVec<String, i32> = MiniVec::new();
    // v3.push(String::from("ssss"));
    // v3.set_id(32);

    println!("{:?}", v);

    let mut v4: MiniVec<i64, i64> = MiniVec::new();
    v4.set_id(10);

    println!("AAA {:?}", use_minivec(&v4));

    let student = Student::new(777);
    use_ided(student);

    let student1 = Student::new(888);
    use_ided1(student1);

    let student2 = Student::new(999);
    use_ided2(Box::new(student2));

    Ok(())
}
