use std::io;

fn main() {
    println!("Welcome to the Temperature Converter");

    // Start app loop
    loop {
        // Get the user's conversion choice
        println!(
            "What would you like to convert?\nCelcius -> Fahrenheit: 1\nFahrenheit -> Celcius: 2\nExit: 3"
        );

        let choice = get_user_input();

        match choice.trim() {
            "1" => {
                println!("Enter the temperature in Celcius: ");
                let temperature = get_user_input();
                let temperature: f32 = temperature.trim().parse().unwrap();
                let converted_temp = (temperature * 9.0 / 5.0) + 32.0;
                println!("{}°C is {}°F", temperature, converted_temp)
            }
            "2" => {
                println!("Enter the temperature in Fahrenheit: ");
                // formular to convert fahrenheit to celcius
                let temperature = get_user_input();
                let temperature: f32 = temperature.trim().parse().unwrap();
                let converted_temp = (temperature - 32.0) * 5.0 / 9.0;
                println!("{}°F is {}°C\n", temperature, converted_temp)
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Please enter a valid choice!");
            }
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}
