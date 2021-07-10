use std::usize;

use crate::ided::Ided;

#[derive(Debug)]
pub struct EmployeeRecords {
    idx: usize,
    employees: Vec<Employee>,
}

impl EmployeeRecords {
    pub fn new() -> EmployeeRecords {
        EmployeeRecords {
            idx: 0,
            employees: Vec::new(),
        }
    }

    pub fn push(&mut self, employee: Employee) {
        &self.employees.push(employee);
    }
}

impl Iterator for EmployeeRecords {
    type Item = Employee;

    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() > self.idx {
            let output = self.employees[self.idx].clone();
            self.idx += 1;
            Some(output)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Employee {
    pub name: String,
    pub id: i64,
}

impl Employee {
    pub fn new(name: String, id: i64) -> Employee {
        Employee { name, id }
    }
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    // pub fn update(&mut self, id: i64) {
    //     self.id = id;
    // }
}

impl Ided for Employee {
    fn my_id(&self) -> i64 {
        self.id()
    }
}
