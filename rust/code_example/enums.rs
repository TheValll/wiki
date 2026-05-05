#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    California,
    Colorado,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({x}, {y})"),
            Message::Write(text) => println!("Write: {text}"),
            Message::ChangeColor(r, g, b) => println!("Change color to ({r}, {g}, {b})"),
        }
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::Arizona => year >= 1912,
            UsState::California => year >= 1850,
            UsState::Colorado => year >= 1876,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    match &home {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {a}.{b}.{c}.{d}"),
        IpAddr::V6(addr) => println!("IPv6: {addr}"),
    }
    match &loopback {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {a}.{b}.{c}.{d}"),
        IpAddr::V6(addr) => println!("IPv6: {addr}"),
    }

    let msg = Message::Write(String::from("hello"));
    msg.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six = {six:?}, none = {none:?}");

    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alaska);
    println!("Penny = {} cents", value_in_cents(&penny));
    println!("Quarter = {} cents", value_in_cents(&quarter));

    let dice_roll = 9;
    match dice_roll {
        3 => println!("Fancy hat!"),
        7 => println!("Remove hat!"),
        other => println!("Move {other} spaces"),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }
}
