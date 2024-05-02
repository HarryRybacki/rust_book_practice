use std::io;

fn main() {
    // prompt user for conversion to C from F or F from C
    let mut conversion_type = String::new();
    println!("Do you wish to convert to 'Celsius' or 'Fahrenheit'?");
    io::stdin().read_line(&mut conversion_type).expect("Failed to read conversion request");

    // prompt user for temp to be converted
    let mut orig_temp = String::new();
    println!("Please enter the temperature to be converted");
    io::stdin().read_line(&mut orig_temp).expect("Failed to read line");

    // remove any tricky spaces or newlines...
    let orig_temp: f32 = orig_temp.trim().parse().expect("Invalid temperature entered... Please try again.\n");

    // remove any tricky spaces or newlines....
    let conversion_type = conversion_type.trim();

    // convert temperature
    match conversion_type {
        "Celsius" => celsius_to_fahrenheit(&orig_temp),
        "Fahrenheit" => fahrenheit_to_celsius(&orig_temp),
        _ => println!("Invalid conversion selected... Please try again.")
    }
}

fn celsius_to_fahrenheit(temp: &f32) {
    println!("Converting {temp} Celsius to Fahrenheit.");
    let converted_temp: f32 = (temp * 9.0 / 5.0) + 32.0;
    println!("{temp} degrees Celsius is {converted_temp} degrees Fahrenheit")
}

fn fahrenheit_to_celsius(temp: &f32) {
    println!("Converting {temp} Fahrenheit to Celsius.");
    let converted_temp: f32 = (temp - 32.0) * 5.0 / 9.0;
    println!("{temp} degrees Fahrenheit is {converted_temp} degrees Celsius")
}