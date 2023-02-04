#[derive(Debug)]
struct Ascii(Vec<u8>);

#[test]
fn test_ascii() {
    let ascii = Ascii(vec![128]);

    println!("{:?}", ascii);
}
