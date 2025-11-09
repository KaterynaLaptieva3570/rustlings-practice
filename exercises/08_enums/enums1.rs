#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Quit,
    Move,
    Echo,
    
    ChangeColor(u8, u8, u8), 
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor(255, 100, 50)); 
    println!("{:?}", Message::Quit);
}