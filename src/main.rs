mod complex;

use complex::Complex;

fn main() {
    let x = Complex::new(5.0, 5.0);
    let y = Complex::new(-5.0, 5.0);
    let z = x + y;
    println!("{}", z);
}
