#![cfg_attr(feature = "no_std", no_std)]

use core::ops::{self, Bound, RangeBounds};

mod error;

pub type Error = GetError;
pub use error::GetError::{
    self, EndIndexOverflowError, IndexError, SliceEndIndexLenError, SliceIndexOrderError,
    SliceStartIndexLenError, StartIndexOverflowError,
};

#[cfg(test)]
mod tests;

pub trait GetCheckedSlice<T: ?Sized>
{
    type Output: ?Sized;
    fn get_checked(self, slice: &T) -> Result<&Self::Output, Error>;
    fn get_checked_mut(self, slice: &mut T) -> Result<&mut Self::Output, Error>;
}

impl<T> GetCheckedSlice<[T]> for usize
{
    type Output = T;

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&T, Error>
    {
        // SAFETY: `self` is checked to be in bounds.
        if self < slice.len()
        {
            unsafe { Ok(&*slice.get_unchecked(self)) }
        }
        else
        {
            Err(IndexError(self, slice.len()))
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut T, Error>
    {
        // SAFETY: `self` is checked to be in bounds.
        if self < slice.len()
        {
            unsafe { Ok(&mut *slice.get_unchecked_mut(self)) }
        }
        else
        {
            Err(IndexError(self, slice.len()))
        }
    }
}

impl<T> GetCheckedSlice<[T]> for ops::Range<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        let len = slice.len();
        if self.start > self.end
        {
            Err(SliceIndexOrderError(self.start, self.end))
        }
        else if self.end > len
        {
            Err(SliceEndIndexLenError(self.end, len))
        }
        else
        {
            unsafe { Ok(&*slice.get_unchecked(self)) }
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], Error>
    {
        let len = slice.len();
        if self.start > self.end
        {
            Err(SliceIndexOrderError(self.start, self.end))
        }
        else if self.end > len
        {
            Err(SliceEndIndexLenError(self.end, len))
        }
        else
        {
            unsafe { Ok(&mut *slice.get_unchecked_mut(self)) }
        }
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeTo<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if end > len => Err(SliceEndIndexLenError(end, len))?,
            | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], Error>
    {
        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if end > len => Err(SliceEndIndexLenError(end, len))?,
            | _ => Ok(unsafe { &mut *slice.get_unchecked_mut(self) }),
        }
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeFrom<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let len = slice.len();

        match slice
        {
            | _ if start > len => Err(SliceStartIndexLenError(start, len))?,
            | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], Error>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let len = slice.len();

        match slice
        {
            | _ if start > len => Err(SliceStartIndexLenError(start, len))?,
            | _ => Ok(unsafe { &mut *slice.get_unchecked_mut(self) }),
        }
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeFull
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        Ok(slice)
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], Error>
    {
        Ok(slice)
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeInclusive<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if start > end => Err(SliceIndexOrderError(start, end))?,
            | _ if end > len => Err(SliceEndIndexLenError(end, len))?,
            | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], Error>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if start > end => Err(SliceIndexOrderError(start, end))?,
            | _ if end > len => Err(SliceEndIndexLenError(end, len))?,
            | _ => Ok(unsafe { &mut *slice.get_unchecked_mut(self) }),
        }
    }
}

impl<T> GetCheckedSlice<[T]> for ops::RangeToInclusive<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], Error>
    {
        (0..=self.end).get_checked(slice)
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], Error>
    {
        (0..=self.end).get_checked_mut(slice)
    }
}

pub trait GetChecked<T>
{
    /// Returns a [`Result`] containing a reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a [`Result`] containing a reference to the element at that
    ///   position or [`Err`] containing a [`GetError`] if out of bounds.
    /// - If given a range, returns a [`Result`] containing the subslice corresponding to that range,
    ///   or [`Err`] containing a [`GetError`] if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use get_checked::GetChecked;
    ///
    /// let v = [10, 40, 30];
    /// assert_eq!(Ok(&40), v.get_checked(1));
    /// assert_eq!(Ok(&[10, 40][..]), v.get_checked(0..2));
    /// assert!(v.get_checked(3).is_err());
    /// assert!(v.get_checked(0..4).is_err());
    /// ```
    #[inline]
    fn get_checked<I>(&self, index: I) -> Result<&I::Output, Error>
    where I: GetCheckedSlice<Self>
    {
        index.get_checked(self)
    }

    /// Returns a [`Result`] containing a mutable reference to an element or subslice depending on the
    /// type of index (see [`get`]) or [`Err`] containing a [`GetError`] if the index is out of bounds.
    ///
    /// [`get`]: slice::get
    ///
    /// # Examples
    ///
    /// ```
    /// use get_checked::GetChecked;
    ///
    /// let x = &mut [0, 1, 2];
    ///
    /// if let Ok(elem) = x.get_checked_mut(1)
    /// {
    ///     *elem = 42;
    /// }
    /// assert_eq!(x, &[0, 42, 2]);
    /// assert!(x.get_checked_mut(3).is_err());
    /// ```
    #[inline]
    fn get_checked_mut<I>(&mut self, index: I) -> Result<&mut I::Output, Error>
    where I: GetCheckedSlice<Self>
    {
        index.get_checked_mut(self)
    }
}

impl<T> GetChecked<T> for [T] {}
