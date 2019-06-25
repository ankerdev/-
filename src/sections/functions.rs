#[allow(dead_code)]
pub fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

pub fn create_heading(section_name: &str) -> std::string::String {
    let padding = "+ + + + + + + +";
    format!("{} {} {}", padding, section_name, padding)
}
