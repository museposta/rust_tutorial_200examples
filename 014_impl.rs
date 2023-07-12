
enum Color {
    Red,
    Green,
}

impl Color {
    fn print(&self){
        match self {
            Color::Red => println!("red"),            
            Color::Green => println!("green"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self){
        println!("The dimensions of the box is {:?}-{:?}-{:?}", self.height, self.width, self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color:Color, dimensions: Dimensions) -> Self
    {
        Self {
            weight,
            color,
            dimensions
        }
    }

    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("TWeight {:?}", self.weight);
    }
}


fn main(){
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };

    let small_box = ShippingBox::new(5.0, Color::Green, small_dimensions);
    small_box.print();
}

