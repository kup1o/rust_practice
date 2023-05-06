use std::io;

fn main() {
    loop {
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read the line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if number % 4 == 0 {
            println!("number is divisible by 4");
        }

        if number % 3 == 0 {
            println!("number is divisible by 3");
        }

        if number % 2 == 0 {
            println!("number is divisible by 2");
        }

        if number % 4 != 0 && number % 3 != 0 && number % 2 != 0 {
            println!("number is not divisible by 4, 3 or 2");
        }

    }
}
