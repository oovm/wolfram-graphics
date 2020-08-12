use wolfram_graphics::ParserConfig;

#[test]
fn test1() {
    let ast = ParserConfig::default();//.parse(include_str!("test1.m")).unwrap();
    println!("{:#?}",ast)
}