// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

// !I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    // data = (&data).to_uppercase(); // this is not the same as `&data.to_uppercase()`. Latter is returning a reference to the String that to_uppercase() returns. I'm so glad I tried using parens around &data, otherwise I'd have never guessed & was being applied to the entire expression.
    //& is not an address. It's better to think of & as borrow based on my understanding so far. Rust anyway corrects you if you give a reference where an actual struct is expected.
    //wait & is overloaded I think. It functions as a borrow, and also as a reference. Maybe a borrow is a reference. Hmmm, I'm sure I can wrap my head around this in just a couple of more uses.
    data = data.to_uppercase();

    println!("{}", data);
}
