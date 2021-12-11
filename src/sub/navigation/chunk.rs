mod parser;

pub use parser::{parse_chunks_for_line, ParseError, ParseResult};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChunkStyle {
    Angle,
    Curly,
    Paren,
    Square,
}

#[derive(Debug)]
pub struct Chunk {
    style: ChunkStyle,
    chunks: Vec<Box<Chunk>>,
}

impl Chunk {
    pub fn new(style: ChunkStyle) -> Chunk {
        Chunk {
            style,
            chunks: vec![],
        }
    }
}
