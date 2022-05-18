
#[test]
fn test_hello_world() {
    let s = hello_world::hello();
    assert_eq!("Hello, world!", s);

    let s = hello_world::hello_world();
    assert_eq!("Hello, world!", s);

    assert_eq!(String::from("Hello, world!"), s);
}
