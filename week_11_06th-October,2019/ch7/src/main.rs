#[macro_use] extern crate text_io;


fn main() {
    // reading from a string source
    let i: i32;
    scan!("<b>12</b>".bytes() => "<b>{}</b>", i);
    assert_eq!(i, 12);

    // reading multiple values from stdio
    let a: i32;
    let b: &mut u8 = &mut 5;
    scan!("{}, {}", a, *b);

}
