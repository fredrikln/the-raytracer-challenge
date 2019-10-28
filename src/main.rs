mod point;
mod vector;

use point::Point;
use vector::Vector;

fn main() {
    let p: Point = Point { x: 1.0, y: 2.0, z: 3.0 };
    let v: Vector = Vector { x: 4.0, y: 5.0, z: 6.0 };

    println!("point: {:?}, vector: {:?}", p, v);
    println!("Hello, world!");
}
