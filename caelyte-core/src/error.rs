use thiserror::Error;

// should have generic erros, divide by 0, parsing, not found, invalid state, etc
#[derive(Error, Debug)]
pub enum CaelyteError {
    #[error("{0}")]
    HexParseError(&'static str),
}

impl From<&'static str> for CaelyteError {
    fn from(msg: &'static str) -> Self {
        CaelyteError::HexParseError(msg)
    }
}
