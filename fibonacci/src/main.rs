use std::io;

fn main() {
    loop {
        println!("Fibonacci generator");
        println!("Note that the first F = 0 is omitted by index.");
        println!("Write an index of a Fibonacci number:");

        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read the line");
        let index: u128 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if index > 186 {
            println!("Index over 186 is not currently supported");
            continue;
        } else {
            println!("Your index is {}", index);
            println!("Your Fibonacci number is {}", fibonacci(index));
        }

        break;
    }
}

fn fibonacci(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }

    let mut fib_nums: Vec<u128> = vec![0, 1];

    for i in 2..=n {
        let fib = fib_nums[(i - 1) as usize] + fib_nums[(i - 2) as usize];
        fib_nums.push(fib);
    }

    fib_nums[n as usize].into()
}
