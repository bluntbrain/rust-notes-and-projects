fn main() {
    let emp_info: (&str,u8) = ("John", 30);
    let (emp_name, emp_age) = emp_info;
    println!("Employee name: {}, age: {}", emp_name, emp_age);
}
