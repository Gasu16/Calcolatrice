use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let x: i32 = args[1].parse().unwrap();

    let y: i32 = args[2].parse().unwrap();

    let operator = &args[3];

    match operator.as_str() {
        "+" => add(x, y),
        "-" => sub(x, y),
        "x" => mul(x, y),
        "/" => div(x, y),
        _   => panic!("Error!"),
    }

}

fn add(x: i32, y: i32) {
    println!("{}", x + y);
}

fn sub(x: i32, y: i32) {
    println!("{}", x - y);
}

fn mul(x: i32, y: i32) {
    println!("{}", x * y);
}

fn div(x: i32, y: i32) {
    println!("{}", x / y);
}
