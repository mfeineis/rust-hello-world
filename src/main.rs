extern crate rand;
extern crate communicator;

use rand::Rng;
use std::cmp::Ordering;
//use std::fmt::Display;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::collections::HashMap;
use std::fs;
use std::fs::File;

use communicator::network;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

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
    network::connect();

    let _f = File::open("hello.txt");

    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error)
        },
    };

    let _f1 = File::open("hello.txt").unwrap();
    let _f2 = File::open("hello.txt").expect("Failed to open hello.txt");

    //let vv = vec![1, 2, 3];
    //vv[99];

    //panic!("crash and burn");

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

    {
        let _v1: Vec<i32> = Vec::new();
        let _v2 = vec![1, 2, 3];
        let mut v3 = Vec::new();
        v3.push(5);
        v3.push(6);
        v3.push(7);
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        let _third: &i32 = &v[2];

        match v.get(2) {
            Some(_) => println!("found: {}", 2),
            None => println!("boom! {}", 2)
        }

        for i in &v {
            println!("{}", i);
        }
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let _row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let _s3 = s1 + &s2; // s1 has been moved and can no longer be used

        //let s4 = format!("{}-{}-{}", s1, s2, s3);
        //println!("s4 = {}", s4);
    }

    {
        let hello = "Здравствуйте";

        let s = &hello[0..4];
        println!("cyrillic slice: {}", s);
    }

    for c in "नमस्ते".chars() {
        println!("{}", c); // 6 chars
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b); // 18 bytes
    }

    // TODO: grapheme clusters via non std create

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let _scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point!

        // println!("field_name = {}", field_name); // Won't compile!
        let team_name = String::from("Blue");
        match map.get(&team_name) {
            Some(_) => println!("Found!"),
            None => println!("Not Found!"),
        }

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        for (key, value) in &scores {
            println!("{} -> {}", key, value);
        }

        // Replace via insert and same key
        scores.insert(String::from("Blue"), 11);

        // Insert if not already present
        scores.entry(String::from("Yellow")).or_insert(51);
    }

    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }

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

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
