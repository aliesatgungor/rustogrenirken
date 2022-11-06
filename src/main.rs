use std::io;
use rand::Rng;
fn main() {
    println!("Hello, world!");
    let sn = rand::thread_rng().gen_range(1..=100);
    println!("Number is {sn}");
    println!("Type Number");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed To read line");

    println!("You guessed {guess}");         



}
