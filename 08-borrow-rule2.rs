// borrowing rule2: can have one mutable references

fn main(){
    let mut s1 = String::from("Hello");
    let mut s2 = &mut s1;
    // let s3 = &s1; cannot use because s2 has mutable reference
    // let s4 = &s1; cannot use because s2 has mutable reference
    // println!("{} {} {} {}",s1,s2,s3,s4);
    // println!("{} {}",s1,s2);
    s2.push_str("World");
}


