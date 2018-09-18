extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let gift = give_ownership();
    println!("{}", gift);

    let s1 = String::from("hello");

    let s2 = take_and_give_back(s1);
    println!("({})", s2);

    let (s3, len) = calculate_length(s2);
    let borrowed_length = borrowed_calculate_length(&s3);
    println!("{} of length {}, borrowed is {}", s3, len, borrowed_length);

    take_ownership(s3);

    let mut s4 = String::from("hello");
    change(&mut s4);

    let _r1 = &s4;
    let _r2 = &s4;
    //let r3 = &mut s4; // boom!

    let _no_dangling_reference = no_dangle();

    let some_words = String::from("hello world");
    let word = first_word(&some_words);
    println!("first word of {} is {}", some_words, word);

    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..=3];

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                for _ in 1..secret_number {
                    print!(".");
                }

                println!();
                println!("You win!");
                break;
            }
        }
    }
}

//fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str { // better
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn borrowed_calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn take_and_give_back(some_string: String) -> String {
    some_string
}

fn give_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
    //let s = String::from("hello");

    let x = 5;
    let y = x;

    println!("({}, {})", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world! {}", s1, s2);
}
