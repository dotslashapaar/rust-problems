fn main(){
    let s1 = String::from("Hello");
    let s2 = s1; // s2 is the owner
    // println!("{}",s1); cannot print because s2 is the owner
    println!("from fn main, s2 is the owner {}", s2);
    print_str(s2); // ownership passed to the fn and s is the owner
    // println!("{}",s2); cannot print because ownership was passed to the fn print_str
}

fn print_str(s: String){ // s is the owner
    println!("from fn print_str{}",s);
}

