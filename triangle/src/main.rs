struct Triangle {
    base: f64,
    height: f64,
}

impl Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height)/2.0
    }
}

fn main() {
    let tri0 = Triangle {base: 5.0, height: 3.0};
    println!("The area is {}", tri0.area() );
}
