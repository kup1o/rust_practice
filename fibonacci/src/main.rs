use std::io;

fn main() {
    loop {
        println!("Fibonacci generator");
        println!("Note that the first F = 0 is ommited by index.");
        println!("Write an index of a fibonacci number:");

        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read the line");
        let index: u32 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your index is {index}");
        println!("Your fibonnaci number is {}", fibonacci(index));

        break;
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
