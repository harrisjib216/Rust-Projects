/*
enums allow us to enummerate a list of variants

when is it appropriate to use enums over structs?
there are two types of IP addresses so using an enum
to express one class of data with different variants works well
*/

enum Message {
    Quit,                       // stores nothing
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // stores string
    ChangeColor(i32, i32, i32), // stores three integers
}

// enum is for a variant: multi variant structs
enum IpAddrVariant {
    V4(String),
    V6(String),

    V4_Data(u8, u8, u8, u8),
}
// struct is for storing data
struct IpAddr {
    variant: IpAddrVariant,
    addr: String,
}
// can use enums as parameters
fn route(variant: IpAddrVariant) {}

// in rust there are no NULL values the option enum holds two variants Some or None
// code description
enum Option_<T> {
    Some(T),
    None,
}

fn main() {
    // variants are namespaced under their identifiers
    let ver4 = IpAddrVariant::V4;
    let ver6 = IpAddrVariant::V6;

    // sample ip address
    let localhost = IpAddrVariant::V4(String::from("127.0.0.1"));
    let localhost = IpAddrVariant::V4_Data(127, 0, 0, 1);

    // rust can infer the types here
    let some_num = Some(5);
    let some_str = Some(String::from("Testing"));

    // rust cannot infer this type w/o the type annotation
    let absent_num: Option<i32> = None;

    // integer and optional integer
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);

    // testing match and enums
    get_numeric_value(Coin::Quater(UsState::Alabama));

    let one = plus_one(Some(0));

    if let Some(1) = one {
        println!("one!");
    } else {
        println!("zero")
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn get_numeric_value(coin: Coin) -> u8 {
    // the match expression lets use compare a value against patterns
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("Got a quater from {:?}!", state);
            25
        }
    }
}

fn plus_one(val: Option<i32>) -> Option<i32> {
    match val {
        Some(i) => Some(i + 1),
        // None => None,
        _ => None,
    }
}
