
/// &'static 静态生命周期符号
pub fn hello() -> &'static str {
    "Hello, world!"
}

pub fn hello_world() -> String {
    String::from("Hello, world!")
}
