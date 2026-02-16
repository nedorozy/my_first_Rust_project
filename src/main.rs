use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret = gener();
    loop {
        let input = input_fn();
        let input = perevod(&input);
        if sravnenie(input, secret) {
            break;
        }
    }
}
fn gener() -> u32 {
    let secret = rand::thread_rng().gen_range(1..=100);
    secret
}
fn input_fn() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Введи что то вменяемое");
    input
}
fn perevod(input: &str) -> u32 {
    let input_num: u32 = input.trim().parse().expect("Введи число");
    input_num
}
fn sravnenie(input: u32, secret: u32) -> bool {
    match input.cmp(&secret) {
        Ordering::Less => {
            println!("мало");
            false
        }
        Ordering::Greater => {
            println!("много");
            false
        }
        Ordering::Equal => {
            println!("Угадал");
            true
        }
    }
}
