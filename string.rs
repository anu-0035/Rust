fn main() {
    //&str fixed length string /special memory
    //String - Dynamic length Strings /heap allocated
    let mut string_literal = String::from("Hello ME!!");
    string_literal.push_str(" Welcome");
    println!("This is string literal : {}",string_literal);

    let s1:&str = "ME";
    println!("{}",s1);
}
