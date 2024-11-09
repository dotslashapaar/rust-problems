fn main(){
    let mut vex = Vec::new();
    vex.push(1);
    vex.push(2);
    vex.push(3);
    vex.push(4);

    is_even(&mut vex);
    println!("{:?}",vex);
}

fn is_even(v: &mut Vec<i32>){
    let mut i = 0;
    while i < v.len() {
        if v[i]%2 != 0 {
            v.remove(i);
        }else{
            i= i+1;
        }
    }
    
}
