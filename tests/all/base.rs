use ressa::Parser;

#[test]
pub fn test() {
    let code = r#"function test() {
        let x = 1;
        let y = 1;
        let z = x + y;
        return z;
    }"#;
    let res = Parser::new(code).unwrap();
    for p in res {
        println!("{:?}", p);
    }
}