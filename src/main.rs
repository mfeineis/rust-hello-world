extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
//use std::fmt::Display;
use std::io;

#[derive(Debug)]
enum Role {
    Admin,
    User,
}

impl Role {
    fn is_special(&self) -> bool {
        match self {
            Role::Admin => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Clone, Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    //roles: Role[],
}

impl User {
    fn associated() -> bool {
        true
    }

    fn disable(&self) -> User {
        let mut new = self.clone();
        new.active = false;

        new
    }
}

fn main() {
    println!("Guess the number!");

    let aang = User {
        email: String::from("aang@tl.ab"),
        username: String::from("Aang"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", aang);

    let inactive_aang = aang.disable();
    println!("{:?}", inactive_aang);

    println!("{}", User::associated());

    let role = Role::Admin;
    println!("{:?}, is_special? {}", role, role.is_special());

    let coin = Coin::Quarter(UsState::Alaska);
    println!("{:?} is worth {} cents", coin, coin.value_in_cents());

    println!("plus_one: None => {:?}, Some(41) => {:?}", plus_one(None), plus_one(Some(41)));

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three")
    }

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
fn first_word(s: &str) -> &str {
    // better
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
