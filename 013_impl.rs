


struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    fn distance_from(&self, other: &Point) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        
        return (dx * dx + dy * dy).sqrt();
    }
}

struct Temperature{
    degrees: f64,
}

impl Temperature{
    // Self with capital S indicates no parameter the same as Temperature
    fn getting_colder()->Self{
        Self{degrees: 80.0}

    }
     // Self with lowercase S indicates parameters passes
    fn show_temp(&self){
        println!("{:?} degrees", self.degrees);
    }
}

fn main() {
    let point_1 = Point::new(10.0, 20.0);
    let point_2 = Point::new(30.0, 40.0);

    println!("The distance between point_1 and point_2 is {}", point_1.distance_from(&point_2));

    let cool = Temperature{degrees: 40.4};

    cool.show_temp();
     


        let hot = Temperature::getting_colder();

        hot.show_temp();


}


// Write some code in rust. Please follow the instructions below.
// a detailed sample with impl keyword.