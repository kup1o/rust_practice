mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use only works in the scope
// use crate::front_of_house::hosting;
// The easiest way is `pub use`
pub use crate::front_of_house::hosting;

mod customer {
    // To fix this problem, move the use within the customer module too
    use crate::front_of_house::hosting;
    pub fn eat_at_restaraunt() {
        // or reference the shortcut in the parent module with super::hosting within the child customer module
        // super::crate::front_of_house::hosting::add_to_waitlist();
        hosting::add_to_waitlist(); // L#14
    }
}

pub fn eat_at_cafe() {
    hosting::add_to_waitlist(); // L#10
}
