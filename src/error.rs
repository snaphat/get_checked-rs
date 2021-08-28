use core::fmt;

use write as w;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error
{
    IndexError(usize, usize),
    SliceIndexOrderError(usize, usize),
    SliceStartIndexLenError(usize, usize),
    SliceEndIndexLenError(usize, usize),
    StartIndexOverflowError(),
    EndIndexOverflowError(),
}

use Error::{
    EndIndexOverflowError, IndexError, SliceEndIndexLenError, SliceIndexOrderError,
    SliceStartIndexLenError, StartIndexOverflowError,
};

impl fmt::Display for Error
{
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            | IndexError(a, b)              => { w!(f, "index out of bounds: the len is {0} but the index is {1}", a, b) },
            | SliceIndexOrderError(a, b)    => { w!(f, "slice index starts at {0} but ends at {1}", a, b) },
            | SliceStartIndexLenError(a, b) => { w!(f, "range start index {0} out of range for slice of length {1}", a, b) },
            | SliceEndIndexLenError(a, b)   => { w!(f, "range end index {0} out of range for slice of length {1}", a, b) },
            | StartIndexOverflowError()     => { w!(f, "attempted to index slice from after maximum usize") },
            | EndIndexOverflowError()       => { w!(f, "attempted to index slice up to maximum usize") },
        }
    }
}

#[cfg(feature = "no_std")]
impl core_error::Error for Error {}

#[cfg(not(feature = "no_std"))]
impl std::error::Error for Error {}
