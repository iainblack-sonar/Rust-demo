use std::process::Command;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

// Security Issue: Hardcoded credentials
const DB_PASSWORD: &str = "admin123!";
const API_KEY: &str = "sk-1234567890abcdef";
const SECRET_TOKEN: &str = "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";

// Code smell: Mutable static variable
static mut GLOBAL_COUNTER: i32 = 0;

fn main() {
    let user_input = std::env::args().nth(1).unwrap_or_default();

    // Demonstrate various issues
    process_user_data(&user_input);
    unsafe_operations();
    complex_function(10, 20, true, false, "test");
    duplicate_logic_one();
    duplicate_logic_two();

    let _unused_variable = 42;  // Code smell: unused variable
    let result = risky_unwraps();
    println!("Result: {:?}", result);
}

// Security Issue: Command injection vulnerability
fn process_user_data(input: &str) {
    // Vulnerable: User input directly in shell command
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("echo {}", input))
        .output()
        .expect("Failed to execute");

    println!("Output: {:?}", output);

    // Vulnerable: SQL-like string concatenation (simulated)
    let query = format!("SELECT * FROM users WHERE name = '{}'", input);
    execute_query(&query);
}

fn execute_query(query: &str) {
    // Simulated database query - demonstrates the vulnerability
    println!("Executing: {}", query);
}

// Security Issue: Unsafe code with potential memory issues
fn unsafe_operations() {
    unsafe {
        GLOBAL_COUNTER += 1;

        // Potential null pointer dereference
        let ptr: *const i32 = std::ptr::null();
        if !ptr.is_null() {
            let _value = *ptr;
        }

        // Raw pointer manipulation
        let mut data = vec![1, 2, 3, 4, 5];
        let ptr = data.as_mut_ptr();
        *ptr.offset(10) = 99;  // Buffer overflow: writing beyond bounds
    }
}

// Code smell: High cyclomatic complexity
fn complex_function(a: i32, b: i32, flag1: bool, flag2: bool, mode: &str) -> i32 {
    let mut result = 0;

    if a > 0 {
        if b > 0 {
            if flag1 {
                if mode == "add" {
                    result = a + b;
                } else if mode == "sub" {
                    result = a - b;
                } else if mode == "mul" {
                    result = a * b;
                } else if mode == "div" {
                    if b != 0 {
                        result = a / b;
                    }
                } else {
                    result = a;
                }
            } else if flag2 {
                result = b;
            } else {
                result = 0;
            }
        } else if b < 0 {
            if flag1 && flag2 {
                result = a - b;
            } else if flag1 || flag2 {
                result = a + b;
            } else {
                result = -1;
            }
        } else {
            result = a;
        }
    } else if a < 0 {
        if b > 0 {
            result = b - a;
        } else {
            result = a + b;
        }
    } else {
        result = b;
    }

    result
}

// Code smell: Duplicated logic (copy 1)
fn duplicate_logic_one() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("alpha".to_string(), 1);
    map.insert("beta".to_string(), 2);
    map.insert("gamma".to_string(), 3);

    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }

    let total: i32 = map.values().sum();
    println!("Total: {}", total);

    map
}

// Code smell: Duplicated logic (copy 2)
fn duplicate_logic_two() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("alpha".to_string(), 1);
    map.insert("beta".to_string(), 2);
    map.insert("gamma".to_string(), 3);

    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }

    let total: i32 = map.values().sum();
    println!("Total: {}", total);

    map
}

// Code smell: Excessive unwrap() calls without error handling
fn risky_unwraps() -> Vec<String> {
    let file_path = "/tmp/data.txt";

    // Multiple unwrap calls that could panic
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<String> = contents
        .lines()
        .map(|s| s.to_string())
        .collect();

    let first = lines.first().unwrap().clone();
    let parsed: i32 = first.parse().unwrap();

    println!("Parsed value: {}", parsed);

    lines
}

// Dead code: Function never called
fn unused_helper_function(x: i32, y: i32) -> i32 {
    let sum = x + y;
    let product = x * y;
    let difference = x - y;

    if sum > 100 {
        product
    } else {
        difference
    }
}

// Security: Insecure random number generation
fn generate_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Weak: Using timestamp for "random" token
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    format!("token_{}", timestamp)
}

// Code smell: Empty exception handling
fn silent_failure() {
    let result = std::fs::read_to_string("/nonexistent/path");
    match result {
        Ok(content) => println!("{}", content),
        Err(_) => {} // Silently ignoring error
    }
}

// Security: Path traversal vulnerability
fn read_user_file(filename: &str) -> Option<String> {
    // Vulnerable: No validation of path traversal
    let path = format!("/var/data/{}", filename);
    std::fs::read_to_string(&path).ok()
}

// Code smell: Magic numbers
fn calculate_shipping(weight: f64, distance: f64) -> f64 {
    let base = 5.99;
    let per_kg = 0.75;
    let per_km = 0.12;
    let tax_rate = 0.0825;
    let handling = 2.50;

    let subtotal = base + (weight * per_kg) + (distance * per_km) + handling;
    subtotal * (1.0 + tax_rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_function() {
        // Incomplete test coverage
        assert_eq!(complex_function(5, 3, true, false, "add"), 8);
    }
}
