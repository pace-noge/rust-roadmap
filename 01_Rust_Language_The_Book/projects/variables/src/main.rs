
mod data_types;

fn main() {
    let mut x = 5;
    println!("The Value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of x is: {}", y);

    // mut and shadowing difference
    let spaces = "     ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // invalid way
    // let mut spaces = "     ";
    // spaces = spaces.len();
    // println!("{}", spaces);

    // Data Types

    data_types::floating_point::run();
    data_types::numeric_operations::run();
    data_types::boolean_type::run();
    data_types::character_type::run();
    data_types::compound_type::run();
    data_types::array_type::run();
}
