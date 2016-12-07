use std::cell::Cell;
fn main() {
    let mut x = 5;
    {
        let y =  &mut x;
        *y*=   5;
        println! ("y = {}", y);
    }
    println!("x = {}", x);
    struct Point {
        x: i32,
         y: Cell<i32>, // nope
    }
    let point = Point {x: 32, y: Cell::new(6)};
    point.y.set(7);
    println!("y = {:?}", point.y);


}