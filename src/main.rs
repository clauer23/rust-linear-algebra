mod complex;
mod vector_ops;

use complex::cmp;

fn main() {

    let x = cmp(1, 2);

    let u1 = vec![cmp(1,0), cmp(1,1), cmp(1,0)];
    let u2 = vec![cmp(0, -1), cmp(1, 0), cmp(1, 1)];

    let v = vector_ops::inner_product(u1, u2);
    
    println!("{}", v);
}
