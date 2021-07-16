pub fn run() {
    let tup = (500, 64, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // accessing tuple element
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("tuple: {}, {}, {}", five_hundred, six_point_four, one);

}