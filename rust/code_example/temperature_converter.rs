use std::io;

fn main() {
    println!("----- TEMPERATURE CONVERTER -----");

    loop{
        let mut unit = String::new();
        let mut temperature = String::new();

        println!("Enter a unit to convert. f or c.");

        io::stdin().read_line(&mut unit).expect("Failed to read line");

        let unit: char = match unit.trim().chars().next() {
            Some(c) => c,
            None => {
                println!("Invalid unit");
                return;
            }
        };

        if unit != 'f' && unit != 'c'{
            println!("Please choose a correct unit. f or c.");
            continue;
        }

        println!("Unit selected {}", unit);

        println!("Enter a temperature in {}", unit);

        io::stdin().read_line(&mut temperature).expect("Failed to read line");

        let temperature : f32 = match temperature.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Temperature insert {}", temperature);

        if unit == 'c' {
            celsius_to_fahrenheit(temperature);
        } else {
            fahrenheit_to_celsius(temperature);
        };

        break;
    }
}

fn celsius_to_fahrenheit(temperature : f32){
    println!("{} celsius is equal to {} fahrenheit", temperature, temperature * 1.8 + 32.0);
}

fn fahrenheit_to_celsius(temperature: f32){
    println!("{} fahrenheit is equal to {} celsius", temperature, (temperature - 32.0) / 1.8);
}
