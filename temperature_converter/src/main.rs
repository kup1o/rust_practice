use std::io;

fn main() {
    loop {
        println!(
            "If you want to convert a temperature from celsius to fahrenheit, write 1\nIf you want to convert a temperature from fahrenheit to celsius, write 2"
        );

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read the line");

        let choice: i8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            println!("You chose to convert a temperature from celsius to fahrenheit.\nWrite a temperature in celsius:");

            let mut celsius = String::new();

            io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read the line");

            let celsius: f32 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let fahrenheit = (celsius * 1.8) + 32.0;

            println!("{celsius}°C = {fahrenheit}°F");
        } else if choice == 2 {
            println!("You chose to convert a temperature from fahrenheit to celsius.\nWrite a temperature in fahrenheit:");

            let mut fahrenheit = String::new();

            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read the line");

            let fahrenheit: f32 = match fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let celsius = (fahrenheit - 32.0) / 1.8;

            println!("{fahrenheit}°F = {celsius}°C");
        } else {
            main();
        }

        break;
    }
}
