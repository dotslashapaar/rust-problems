fn main(){
    let my_string = String::from("My name is log1c");
    println!("{}",get_string_length(&my_string));
    println!("{}",my_string);
}


fn get_string_length(s: &String)-> usize {
    s.chars().count()
}
