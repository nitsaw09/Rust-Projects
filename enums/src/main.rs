// Enum is a combination of different variant types like Unit, tuple, struct, enum
enum WebEvent {
    Pageload,
    PageUnLoad,
    KeyPress(char),
    Paste(String),
    Click {
        x: i64,
        y: i64
    }
}

enum Option<T> {
    Some(T),
    None
}

fn main() {
    let quit = WebEvent::KeyPress('q');

    let option = Option::Some(1);
}
