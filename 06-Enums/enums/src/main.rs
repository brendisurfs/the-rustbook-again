enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum MessageKind {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl MessageKind {
    fn some_fn() {
        println!("Rust is sick");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}
fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
}

fn route(ipkind: IpAddrKind) {}
