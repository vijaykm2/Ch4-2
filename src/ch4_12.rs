struct Point {
    x: i32,
    y: i32
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32
}

fn main() {
    let mut point = Point {x: 10, y: 100};
    point.x = 13;
    println!("Point is at ({}, {})", point.x, point.y);


    let mut point = Point { x: 0, y: 0 };

    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };

        *r.x = 5;
        *r.y = 6;
    }

    assert_eq!(5, point.x);
    assert_eq!(6, point.y);

    let mut point2 =  Point3D {x: 10, y:20, z: 30};
    print!("point 2 is {}, {}, {}", point2.x, point2.y, point2.z);

    point2 = Point3D {z: 40, .. point2};
    print!("point 2 is {}, {}, {}", point2.x, point2.y, point2.z);

}