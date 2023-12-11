// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use crate::delicious_snacks::{fruits::PEAR, veggies::CUCUMBER};

mod delicious_snacks {
    // TODO: Fix these use statements
    use self::fruits::PEAR;
    use self::veggies::CUCUMBER;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    let pear = PEAR;
    let cucumber = CUCUMBER;
    println!("favorite snacks: {} and {}", pear, cucumber);
}
