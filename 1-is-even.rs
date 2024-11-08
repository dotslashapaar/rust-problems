fn main(){
    let mut num: i32 = 5;
    println!("{}",is_even(&mut num));
    println!("{}",num);
}

fn is_even(n: &mut i32)-> bool {
    
    if *n%2 == 0 {
        println!("Is Even");
        true
    }else{
        println!("Is Odd");
        false
    }
}
