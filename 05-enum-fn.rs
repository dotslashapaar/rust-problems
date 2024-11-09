enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main(){
    let rect = Shape::Rectangle(2.0, 4.0);
    println!("{}",calculate_area(rect));
    let cir = Shape::Circle(1.0);
    println!("{}",calculate_area(cir));
}

fn calculate_area(shape: Shape)-> f64 {
    match shape{
        Shape::Circle(r)=> 3.14 * r*r,
        Shape::Rectangle(a,b )=> a*b,
    }
}
