enum Message {
    Quir,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct Write(String);
struct ChangeColorMeaagage(i32, i32, i32);

impl Message {
    fn call(&self) {
        // method body
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
