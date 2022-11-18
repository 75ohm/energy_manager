use std::env;

// This is the main function
fn main() {

    for (index, argument) in env::args().enumerate() {
        println!("argument {index} is {argument}");
    }
    if env::args().len() >= 2 {
        let arg2 = env::args().nth(2).unwrap();
        println!("arg2 is {arg2}");
    }
    else {
        println!("Less than 2 arguments were given!")
    }

}
