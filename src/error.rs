use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error
{
    #[error("index {0} out of range for slice of length {1}")]
    IndexError(usize, usize),
    #[error("range start index {0} out of range for slice of length {1}")]
    StartIndexError(usize, usize),
    #[error("range end index {0} out of range for slice of length {1}")]
    EndIndexError(usize, usize),

    #[error("attempted to index slice from after maximum usize")]
    StartIndexOverflowError(),
    #[error("attempted to index slice from after maximum usize")]
    EndIndexOverflowError(),
}
