use std::io;

const UNITS: [&str; 6] = ["km", "miles", "kg", "lbs", "°C", "°F"];

fn display_start(){
    println!("Please choose an option:");
    println!("1 : km → miles");
    println!("2 : miles → km");
    println!("3 : kg → lbs");
    println!("4 : lbs → kg");
    println!("5 : °C → °F");
    println!("6 : °F → °C");
}

fn check_choice(num: u32) -> bool { num > 0 && num < 7 }

fn km_to_miles(value: f64, unit: &str) -> f64 {
    if unit == "km" {
        value * 0.621371
    } else {
        value / 0.621371
    }
}

fn kg_to_lbs(value: f64, unit: &str) -> f64 {
    if unit == "kg" {
        value * 2.20462
    } else {
        value / 2.20462
    }
}

fn celsius_to_fahrenheit(value: f64, unit: &str) -> f64 {
    if unit == "°C" {
        value * 1.8 + 32.0
    } else {
        (value - 32.0) / 1.8
    }
}

fn main() {
      println!("----- UNIT CONVERTER -----");
      let mut choice = String::new();
      let mut value = String::new();

      let chosen: u32;
      let parsed_value: f64;

      loop {
          display_start();
          choice.clear();
          io::stdin().read_line(&mut choice).expect("Failed to read line");

          let c: u32 = match choice.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
          };

          if !check_choice(c) { continue; }

          println!("You chose: {}", c);
          chosen = c;
          break;
      }

      loop {
          let unit = UNITS[(chosen - 1) as usize];
          println!("Enter a {} value", unit);
          value.clear();
          io::stdin().read_line(&mut value).expect("Failed to read line");

          let v: f64 = match value.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
          };

          println!("You chose: {} {}", v, unit);
          parsed_value = v;
          break;
      }

    let unit = UNITS[(chosen - 1) as usize];
    let result = match chosen {
        1 | 2 => km_to_miles(parsed_value, unit),
        3 | 4 => kg_to_lbs(parsed_value, unit),
        5 | 6 => celsius_to_fahrenheit(parsed_value, unit),
        _ => unreachable!(),
    };

    println!("Your conversion result: {}", result);

}
