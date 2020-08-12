use wolfram_pest::Rule;

#[derive(Debug, Clone)]
pub enum Error {
    ParseError(String),
    FileNotFound,
    UnknownError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<wolfram_pest::Error<Rule>> for Error {
    fn from(e: wolfram_pest::Error<Rule>) -> Self {
        Self::ParseError(format!("{}", e))
    }
}