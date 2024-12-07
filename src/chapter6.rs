use std::option;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call_me(&self){
        if matches!(self, Self::Quit) {
            println!("THIS IS QUIT");
        } else {
            println!("this is NOT quit")
        }
    }
}

pub fn call_message(){
    let m = Message::Write(String::from("hello"));
    m.call_me();
}

pub fn use_option() {
    let n : Option<u32> = Some(9);
    if n.is_some(){
        println!("THIS IS HENRE BECAUSE {} is a number", n.unwrap());
    } else {
        println!("n IS NOT A NUMBER");
    }
}

enum Color {
    Green,
    Orange,
    Purple
}

pub fn guess(){
    let o : Color = Color::Green;
    match o {
        Color::Green => println!("CORRECT!"),
        Color::Orange => {
            println!("INCORRECT!");
        },
        Color::Purple => println!("INCORRECT!")
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

pub fn times_two(number : Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(i) => Some(i * 2)
    }
}

// matches are exhaustive, so they have to cover all possible cases, even through a catch-all like 
// a named variable or "_"


pub fn use_if_let() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
    println!("The count is {count}");
}