use std::ops::{self, Bound, RangeBounds};

mod error;
use error::Error;

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
            Err(Error::IndexError(self, slice.len()))
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
            Err(Error::IndexError(self, slice.len()))
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
            Err(Error::SliceIndexOrderError(self.start, self.end))
        }
        else if self.end > len
        {
            Err(Error::SliceEndIndexLenError(self.end, len))
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
            Err(Error::SliceIndexOrderError(self.start, self.end))
        }
        else if self.end > len
        {
            Err(Error::SliceEndIndexLenError(self.end, len))
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
            | Bound::Included(x) => x.checked_add(1).ok_or(Error::EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if end > len => Err(Error::SliceEndIndexLenError(end, len))?,
            | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], Error>
    {
        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(Error::EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if end > len => Err(Error::SliceEndIndexLenError(end, len))?,
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
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error::StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let len = slice.len();

        match slice
        {
            | _ if start > len => Err(Error::SliceStartIndexLenError(start, len))?,
            | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], Error>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error::StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let len = slice.len();

        match slice
        {
            | _ if start > len => Err(Error::SliceStartIndexLenError(start, len))?,
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
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error::StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(Error::EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if start > end => Err(Error::SliceIndexOrderError(start, end))?,
            | _ if end > len => Err(Error::SliceEndIndexLenError(end, len))?,
            | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], Error>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error::StartIndexOverflowError())?,
            | Bound::Unbounded => 0,
        };

        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(Error::EndIndexOverflowError())?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if start > end => Err(Error::SliceIndexOrderError(start, end))?,
            | _ if end > len => Err(Error::SliceEndIndexLenError(end, len))?,
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
    #[inline]
    fn get_checked<I>(&self, index: I) -> Result<&I::Output, Error>
    where I: GetCheckedSlice<Self>
    {
        index.get_checked(self)
    }

    #[inline]
    fn get_checked_mut<I>(&mut self, index: I) -> Result<&mut I::Output, Error>
    where I: GetCheckedSlice<Self>
    {
        index.get_checked_mut(self)
    }
}

impl<T> GetChecked<T> for [T] {}
