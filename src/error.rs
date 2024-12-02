use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day1Error {
    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Split error")]
    Split(#[from] SplitIntError),
}

#[derive(Error, Debug)]
pub enum SplitIntError {
    #[error("First split error")]
    FirstSplitError,
    #[error("Second split error")]
    SecondSplitError,
}
