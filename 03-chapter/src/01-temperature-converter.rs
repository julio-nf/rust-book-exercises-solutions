fn main() {
    let celsius_number = 35.0;
    let fahrenheit_number = 95.0;

    let celsius_converted = convert_to_fahrenheit(celsius_number);
    let fahrenheit_converted = convert_to_celsius(fahrenheit_number);

    println!("Converted to fahrenheit: {}°F", celsius_converted);
    println!("Converted to celsius: {}°C", fahrenheit_converted);
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
