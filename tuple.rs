fn main() {
    //tuple
    let mut emp_info:(&str,u8) = ("Anubhav",21);

    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    println!("Employee Name: {emp_name}, Employee Age: {emp_age}");
    emp_info.1 = 22;
    let emp_age2 = emp_info.1;
    println!("Employee Name: {emp_name} ,Employee Age: {emp_age2}");

    // destructuting
    let (name,age) = emp_info;
    println!("Employee Name: {name} ,Employee Age: {age}");
}
