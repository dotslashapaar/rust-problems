fn main(){
    let vec1 = vec![1,2,3,4,5];
    let vec2: Vec<i32> = vec1.iter().filter(|x| *x % 2 != 0).map(|x| x*2).collect();
    for i in vec2.iter(){
        println!("{}",i);
    }
    println!("{:?}",vec2);
}
