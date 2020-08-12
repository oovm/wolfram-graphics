mod parser;
use std::collections::HashMap;



#[derive(Clone, Debug)]
pub enum AST {
    Graphics(Vec<AST>, HashMap<AST, AST>),
    //
    Integer(u32),
    Decimal(f32),
    List(Vec<AST>),

    //
    Point(Vec<AST>),
    Line(Vec<AST>),
    Circle(Vec<AST>),
    Disk(Vec<AST>),
    Rectangle(Vec<AST>),
    Polygon(Vec<AST>),

    //
    Color(),
    Unknown,
}

#[derive(Clone, Debug)]
pub struct ParserConfig {

}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {}
    }
}