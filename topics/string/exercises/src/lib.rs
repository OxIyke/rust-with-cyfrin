pub fn hello() -> String {
    return String::from("Hello Rust");
}

pub fn greet(name: &str) -> String {
    return String::from("Hello ") + name;
}

pub fn append(mut s: String) -> String {
    s.push('!');
    s
}
