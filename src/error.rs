use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error
{
    #[error("index out of bounds: the len is {0} but the index is {1}")]
    IndexError(usize, usize),
    #[error("slice index starts at {0} but ends at {1}")]
    SliceIndexOrderError(usize, usize),
    #[error("range start index {0} out of range for slice of length {1}")]
    SliceStartIndexLenError(usize, usize),
    #[error("range end index {0} out of range for slice of length {1}")]
    SliceEndIndexLenError(usize, usize),

    #[error("attempted to index slice from after maximum usize")]
    StartIndexOverflowError(),
    #[error("attempted to index slice up to maximum usize")]
    EndIndexOverflowError(),
}
