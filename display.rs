use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

// implement `Display` trait for `Complex`
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let complex = Complex {real: 3.3, imag: 7.2};
    println!("Display: {}", complex);
    println!("Debug: {:#?}", complex);
}