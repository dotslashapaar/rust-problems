pub trait Summary{
    fn summarize(&self)-> String{
        return String::from("hi there"); // default case for trait
    }
}

struct User{
    name: String,
    age: u32,
}

impl Summary for User{
    fn summarize(&self)-> String {
        return format!("The name is {}, and the age is {}", self.name, self.age); // if we remove this default case will work
    }
}


fn main(){
    let user = User{
        name: String::from("Apaar"),
        age: 21,
    };
    println!("{}", user.summarize());
}
    
