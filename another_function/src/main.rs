fn main() {
    print_labeled_measurement(5, 'h');
    rust_is_a_statement_language();

    let x = five();
    println!("The value of x is {x}");

    let z = plus_one(5);
    println!("The value of z is {z}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn rust_is_a_statement_language() {
    // works in C & Ruby
    // x = y = 6
    // but does not in Rust

    // in Rust i would use something like this below
    // let y = {
    //     let x = 3;
    //     x+0
    // };

    // or this

    let x = 3;
    let y = x;

    println!("The value of y is {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
