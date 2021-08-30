use core::fmt;

use write as w;

/// An error that can be returned when using `get_checked` or `get_checked_mut` to retrieve a
/// reference to an element or slice.
///
/// The following traits provide these methods:
/// * [`GetChecked`]
/// * [`GetCheckedSliceIndex`]
///
/// # Causes:
/// `IndexError` is thrown if the index or range is out of bounds and would have otherwise
/// caused a [`panic`] had indexing been performed using [`core::ops::Index`] directly.
///
/// A [`print`] of a given error will match the error message that panic would have produced for
/// the same index or range.
///
/// # Examples
/// ```
/// # use get_checked::GetChecked;
/// let v = vec![1, 2, 3];
/// if let Err(e) = v.get_checked(2..5)
/// {
///     println!("Index error: {}", e);
/// }
///
/// # use get_checked::GetCheckedSliceIndex;
/// let v = vec![1, 2, 3];
/// if let Err(e) = (2..5).get_checked(&v)
/// {
///     println!("Index error: {}", e);
/// }
/// ```
///
/// [`GetChecked`]:           crate::GetChecked
/// [`GetCheckedSliceIndex`]: crate::GetCheckedSliceIndex
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexError
{
    pub(super) kind: IndexErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Enum representing the kind of index error that occurred. These represent the types of
/// diffent types of [`panics`] that could occur if indexing was performed using [`core::ops::Index`].
///
/// [`panics`]: panic
pub enum IndexErrorKind
{
    /// Index is out of bounds.
    /// * `0` - index of element.
    /// * `1` - length of slice.
    Bounds(usize, usize),

    /// Slice index start is after the end of the slice.
    /// * `0` - start of slice.
    /// * `1` - end of slice.
    Order(usize, usize),

    /// Range start is after the end of the slice.
    /// * `0` - start of slice.
    /// * `1` - end of slice.
    StartRange(usize, usize),

    /// Range end is before the start of the slice.
    /// * `0` - start of slice.
    /// * `1` - end end of slice.
    EndRange(usize, usize),

    /// Slice start is after [`usize::MAX`].
    StartOverflow(),

    /// Slice end is at [`usize::MAX`].
    EndOverflow(),
}

use IndexErrorKind::{Bounds, EndOverflow, EndRange, Order, StartOverflow, StartRange};

impl IndexError
{
    /// Outputs the detailed cause of an index error.
    pub fn kind(&self) -> &IndexErrorKind
    {
        &self.kind
    }

    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
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
