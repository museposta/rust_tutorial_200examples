


// trait. circle triangle has area
struct Triangle {
    base: f64,
    height: f64,
}
struct Circle{
    circumference: f64,    
}
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14 *  (self.circumference * self.circumference)
    }
}
impl HasArea for Triangle {
    fn area(&self) -> f64 {
        0.5 * (self.base * self.height)
    }
}
fn main() {
    let a = Triangle {
        base: 10.5,
        height: 17.4,
    };
    let triangle_area = a.area();
    println!("Area of a triangle is {}", triangle_area);

    let a = Circle {
        circumference: 12.2,
    };
    let circle_area = a.area();
    println!("Area of a circle is {}", circle_area);

}