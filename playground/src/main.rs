#[derive(Debug)]
enum State {
    Loading,
    Loaded,
    Error,
}

fn get_state() -> State {
    return State::Loading;
}

fn main() {
    println!("Hello, world!")
}
