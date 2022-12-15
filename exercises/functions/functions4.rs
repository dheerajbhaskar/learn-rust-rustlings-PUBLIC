// functions4.rs
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a hint.

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// (Don't worry about the function bodies themselves, we're only interested
// in the signatures for now. If anything, this is a good way to peek ahead
// to future exercises!)

// !I AM NOT DONE

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

// lol I did do the i32 first. u32 was just me playing around :D
fn sale_price(price: i32) -> u32 {
    use std::convert::TryInto;
    if is_even(price) {
        (price - 10).try_into().unwrap()
    } else {
        (price - 3).try_into().unwrap()
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
