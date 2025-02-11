use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.truncate(line.trim_end().len());
    line
}

enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

fn read_temperature_unit() -> TemperatureUnit {
    println!("Enter the temperature unit:");
    loop {
        let unit = read_line();
        if unit == "C" {
            return TemperatureUnit::Celsius;
        } else if unit == "F" {
            return TemperatureUnit::Fahrenheit;
        } else {
            println!("Unknown temperature unit, please enter 'C' or 'F'.");
        }
    }
}

fn read_number_of_degrees() -> f32 {
    println!("Enter the degrees:");
    loop {
        let degree = read_line();
        match degree.trim().parse() {
            Ok(num) => {
                return num;
            }
            Err(_) => {
                println!("Unknown degree value, please enter 'F' or 'C'.");
            }
        }
    }
}

fn main() {
    let scale = read_temperature_unit();
    let temp = read_number_of_degrees();

    match scale {
        TemperatureUnit::Celsius => println!("{} degrees F", 1.8 * temp + 32.),
        TemperatureUnit::Fahrenheit => println!("{} degrees C", 5. * (temp - 32.) / 9.),
    }
}
