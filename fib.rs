fn main(){
    println!("{}",fib(3));
}

fn fib(n: i32)-> i32{
    let mut first = 0;
    let mut second = 1;

    if n == 0{
        return 0;
    } 
    if n == 1{
        return 1;
    }

    for _ in 1..n{
        let temp: i32 = second;
        second = first + second;
        first = temp;
    }
    return second;

}
