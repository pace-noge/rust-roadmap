#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn run() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixel.", area(width1, height1));
    println!("The area1 {}", area1((width1, height1)));
    println!("rect1 {:?}", rect1);
    println!("The area using struct {}", area2(rect1));
    

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// refactor with tuple

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// using struct

fn area2(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}