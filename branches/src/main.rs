use std::io;
fn main() {
    fn convert_to_celcius(a: f32) -> f32{
        (a - 32.0) / 1.8
    }   
    let mut degree = String::new();
    println!("pleas enter the degree in farenhait");
        io::stdin()
        .read_line(&mut degree)
        .expect("Failed to read line");

    let mut degree: f32 = degree
        .trim()
        .parse()
        .expect("Index entered was not a number");

    degree = convert_to_celcius(degree);
    println!("the degree is {degree} celcius")
}