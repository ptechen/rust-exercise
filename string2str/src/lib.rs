#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let s = "Hello, world!";
        let h = String::from("Hello, world!");
        if s == h {
            println!("{}", true);
        }
        if s == &h {
            println!("{}", true);
        }

        let s = &h[0..];
        println!("{}", s);
        let s = &h[0..2];
        println!("{}", s);
    }
}
