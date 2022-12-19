// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

// !I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move{x:i32, y:i32},
    Echo(String),
    ChangeColor (u8, u8, u8),
    Quit

}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    //dheeraj: takeaway - enums can have any of the types of structs - unit, tuple, c-like struct, and also primitive values I think.
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    //dheeraj: you can use a borrow (&messages) or an owned messages thing as well here. I think I'm starting to understand borrows and owned structs. These are almost nothing like C pointers. In fact, on the internet, these are called smart pointers because they 1. enable borrow checker to work 2. They do somethings differently I think. For example, I didn't have to do a *message to read it. Code remains the same if I do for..in messages or for..in &messages (Takeaway, & is communicating that it's a read-only borrow, not really that it's an address). *message is in fact a way IIUC to unbox the smart pointer to get at the actual value, for example to update it.
    for message in messages {
        message.call();
    }
}
