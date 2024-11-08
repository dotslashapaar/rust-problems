fn main(){
    
    let rectangle = Rect{
        len: 10,
        width: 5
    };

    println!("Area of Rectangle is: {} \nPerimeter of Rectangle is: {}", rectangle.area(), rectangle.perimeter());

}

struct Rect{
    len: i32,
    width: i32
}

impl Rect{

    fn area(&self)-> i32 {
        &self.len * &self.width
    }

    fn perimeter(&self)-> i32 {
        2 * (&self.len + &self.width)
    }

}

