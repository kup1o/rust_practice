fn main() {
    // let x=5;
    // E0384 => you cannot change an immutable variable. all variables in rust are immutable by default
    let mut x = 5;
    println!("The value of x is {x}.");
    x = 6;
    println!("The value of x is {x}.");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours equal {THREE_HOURS_IN_SECONDS} seconds.");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");

    // shadowing instead of mutable variables can change its type // E0308
    let spaces = "     ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces in the variable.");
}
