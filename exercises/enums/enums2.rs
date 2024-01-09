// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    Move {x: i32, y: i32},
    Echo {message: String},
    Color (i32, i32, i32),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }

    fn Echo(message: String) -> Message {
        Message::Echo {message}
    }

    fn ChangeColor(red: i32, green: i32, blue: i32) -> Message {
        Message::Color(red, green, blue)
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
