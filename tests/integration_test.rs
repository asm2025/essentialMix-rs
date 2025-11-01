// Integration tests following TDD principles
// These tests can be run with: cargo test

use serde::{Deserialize, Serialize};

// Common test utilities
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Employee {
    id: u32,
    #[serde(rename = "employee_name")]
    name: String,
    #[serde(rename = "employee_age")]
    age: u8,
    email: Option<String>,
    phone: Option<String>,
    #[serde(rename = "profile_image")]
    image: Option<String>,
    #[serde(rename = "employee_salary")]
    salary: Option<f64>,
}

pub fn get_employees(count: usize) -> Vec<Employee> {
    let n = if count < 1 { 1 } else { count };
    let mut employees: Vec<Employee> = Vec::with_capacity(n);

    for i in 1..=n {
        let name = format!("Employee {}", i);
        employees.push(Employee {
            id: i as u32,
            name,
            age: (i % 100) as u8,
            email: Some(format!("{}@example.com", i)),
            phone: Some(format!("+1-555-555-{:04}", i)),
            image: Some(format!("https://i.pravatar.cc/150?img={}", i)),
            salary: Some((i * 1000) as f64),
        });
    }

    employees
}

// Include all test modules
mod app_tests;
mod random_tests;
mod io_tests;
mod web_tests;
mod threading_tests;
mod log_tests;
mod manual_tests;

