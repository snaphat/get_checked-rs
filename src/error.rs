use core::fmt;

use write as w;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexError
{
    pub(super) kind: IndexErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]

pub enum IndexErrorKind
{
    Bounds(usize, usize),
    Order(usize, usize),

    // Slice Out of ranges errors:
    StartRange(usize, usize),
    EndRange(usize, usize),

    // Slice Overflow errors
    StartOverflow(),
    EndOverflow(),
}

use IndexErrorKind::{Bounds, EndOverflow, EndRange, Order, StartOverflow, StartRange};

impl IndexError
{
    pub fn kind(&self) -> &IndexErrorKind
    {
        &self.kind
    }

    #[rustfmt::skip]
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self.kind
        {
            | Bounds(a, b)     => { w!(f, "index out of bounds: the len is {0} but the index is {1}", a, b) },
            | Order(a, b)      => { w!(f, "slice index starts at {0} but ends at {1}", a, b) },
            | StartRange(a, b) => { w!(f, "range start index {0} out of range for slice of length {1}", a, b) },
            | StartOverflow()  => { w!(f, "attempted to index slice from after maximum usize") },
            | EndRange(a, b)   => { w!(f, "range end index {0} out of range for slice of length {1}", a, b) },
            | EndOverflow()    => { w!(f, "attempted to index slice up to maximum usize") },
        }
    }
}

impl fmt::Display for IndexError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        self.fmt(f)
    }
}

#[cfg(feature = "no_std")]
impl core_error::Error for IndexError {}

#[cfg(not(feature = "no_std"))]
impl std::error::Error for IndexError {}
