use std::ops::{self, Bound, RangeBounds};

mod error;
use error::Error;

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

#[cfg(test)]
mod test
{
    use super::{Error, GetChecked};

    // Immutable tests:

    #[test]
    fn immut_index()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = *bytes.get_checked(4).unwrap();
        assert_eq!(ret, bytes[4]);
    }

    #[test]
    fn immut_index_edge()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = *bytes.get_checked(15).unwrap();
        assert_eq!(ret, bytes[15]);
    }

    #[test]
    fn immut_index_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let err = bytes.get_checked(16).unwrap_err();

        assert_eq!(err.to_string(), "index out of bounds: the len is 16 but the index is 16");
    }

    #[test]
    fn immut_range_full()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(..).unwrap();
        assert_eq!(ret.len(), 16);
        assert_eq!(ret, bytes);
    }

    #[test]
    fn immut_range_full_exclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(2..5).unwrap();
        assert_eq!(ret.len(), 3);
        assert_eq!(ret, &bytes[2..5]);
    }

    #[test]
    fn immut_range_full_exclusive_edge()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(2..16).unwrap();
        assert_eq!(ret.len(), 14);
        assert_eq!(ret, &bytes[2..16]);
    }

    #[test]
    fn immut_range_full_inclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(2..=5).unwrap();
        assert_eq!(ret.len(), 4);
        assert_eq!(ret, &bytes[2..=5]);
    }

    #[test]
    fn immut_range_full_inclusive_edge()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(2..=15).unwrap();
        assert_eq!(ret.len(), 14);
        assert_eq!(ret, &bytes[2..=15]);
    }

    #[test]
    fn immut_range_full_zero_exclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(0..0).unwrap();
        assert_eq!(ret.len(), 0);
        assert_eq!(ret, &bytes[0..0]);
    }

    #[test]
    fn immut_range_full_zero_inclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(0..=0).unwrap();
        assert_eq!(ret.len(), 1);
        assert_eq!(ret, &bytes[0..=0]);
    }

    #[test]
    fn immut_range_from()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(5..).unwrap();
        assert_eq!(ret.len(), 16 - 5);
        assert_eq!(ret, &bytes[5..]);
    }

    #[test]
    fn immut_range_from_zero()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(16..).unwrap();
        assert_eq!(ret.len(), 0);
        assert_eq!(ret, &bytes[16..]);
    }

    #[test]
    fn immut_range_to_exclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(..5).unwrap();
        assert_eq!(ret.len(), 5);
        assert_eq!(ret, &bytes[..5]);
    }

    #[test]
    fn immut_range_to_inclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let ret = bytes.get_checked(..=5).unwrap();
        assert_eq!(ret.len(), 6);
        assert_eq!(ret, &bytes[..=5]);
    }

    #[test]
    fn immut_range_from_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(17..).unwrap_err();
        assert_eq!(err.to_string(), "range start index 17 out of range for slice of length 16");
    }

    #[test]
    fn immut_range_to_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(..17).unwrap_err();
        assert_eq!(err.to_string(), "range end index 17 out of range for slice of length 16");
    }

    #[test]
    fn immut_range_to_inclusive_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(..=16).unwrap_err();
        assert_eq!(err.to_string(), "range end index 17 out of range for slice of length 16");
    }

    #[test]
    fn immut_range_from_slice_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(17..5).unwrap_err();
        assert_eq!(err.to_string(), "slice index starts at 17 but ends at 5");
    }

    #[test]
    fn immut_range_slice_inclusive_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(17..=4).unwrap_err();
        assert_eq!(err.to_string(), "slice index starts at 17 but ends at 5");
    }

    #[test]
    fn immut_range_overflow_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked(0..=usize::MAX).unwrap_err();
        assert_eq!(err.to_string(), Error::EndIndexOverflowError().to_string());
    }

    // Mutable tests:

    #[test]
    fn mut_index()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = *bytes2.get_checked_mut(4).unwrap();
        assert_eq!(ret, bytes[4]);
    }

    #[test]
    fn mut_index_edge()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = *bytes2.get_checked_mut(15).unwrap();
        assert_eq!(ret, bytes[15]);
    }

    #[test]
    fn mut_index_error()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let err = bytes2.get_checked_mut(16).unwrap_err();
        assert_eq!(err.to_string(), "index out of bounds: the len is 16 but the index is 16");
    }

    #[test]
    fn mut_range_full()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(..).unwrap();
        assert_eq!(ret.len(), 16);
        assert_eq!(ret, bytes);
    }

    #[test]
    fn mut_range_full_exclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(2..5).unwrap();
        assert_eq!(ret.len(), 3);
        assert_eq!(ret, &bytes[2..5]);
    }

    #[test]
    fn mut_range_full_exclusive_edge()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(2..16).unwrap();
        assert_eq!(ret.len(), 14);
        assert_eq!(ret, &bytes[2..16]);
    }

    #[test]
    fn mut_range_full_inclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(2..=5).unwrap();
        assert_eq!(ret.len(), 4);
        assert_eq!(ret, &bytes[2..=5]);
    }

    #[test]
    fn mut_range_full_inclusive_edge()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(2..=15).unwrap();
        assert_eq!(ret.len(), 14);
        assert_eq!(ret, &bytes[2..=15]);
    }

    #[test]
    fn mut_range_full_zero_exclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(0..0).unwrap();
        assert_eq!(ret.len(), 0);
        assert_eq!(ret, &bytes[0..0]);
    }

    #[test]
    fn mut_range_full_zero_inclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(0..=0).unwrap();
        assert_eq!(ret.len(), 1);
        assert_eq!(ret, &bytes[0..=0]);
    }

    #[test]
    fn mut_range_from()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(5..).unwrap();
        assert_eq!(ret.len(), 16 - 5);
        assert_eq!(ret, &bytes[5..]);
    }

    #[test]
    fn mut_range_from_zero()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(16..).unwrap();
        assert_eq!(ret.len(), 0);
        assert_eq!(ret, &bytes[16..]);
    }

    #[test]
    fn mut_range_to_exclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(..5).unwrap();
        assert_eq!(ret.len(), 5);
        assert_eq!(ret, &bytes[..5]);
    }

    #[test]
    fn mut_range_to_inclusive()
    {
        let bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];
        let mut bytes2 = bytes.clone();

        let ret = bytes2.get_checked_mut(..=5).unwrap();
        assert_eq!(ret.len(), 6);
        assert_eq!(ret, &bytes[..=5]);
    }

    #[test]
    fn mut_range_from_error()
    {
        let mut bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked_mut(17..).unwrap_err();
        assert_eq!(err.to_string(), "range start index 17 out of range for slice of length 16");
    }

    #[test]
    fn mut_range_to_error()
    {
        let mut bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked_mut(..17).unwrap_err();
        assert_eq!(err.to_string(), "range end index 17 out of range for slice of length 16");
    }

    #[test]
    fn mut_range_to_inclusive_error()
    {
        let mut bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked_mut(..=16).unwrap_err();
        assert_eq!(err.to_string(), "range end index 17 out of range for slice of length 16");
    }

    #[test]
    fn mut_range_slice_error()
    {
        let mut bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked_mut(17..5).unwrap_err();
        assert_eq!(err.to_string(), "slice index starts at 17 but ends at 5");
    }

    #[test]
    fn mut_range_slice_inclusive_error()
    {
        let mut bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked_mut(17..=4).unwrap_err();
        assert_eq!(err.to_string(), "slice index starts at 17 but ends at 5");
    }

    #[test]
    fn mut_range_overflow_error()
    {
        let mut bytes = vec![
            0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5,
            0x3E, 0x1F,
        ];

        let err = bytes.get_checked_mut(0..=usize::MAX).unwrap_err();
        assert_eq!(err.to_string(), Error::EndIndexOverflowError().to_string());
    }
}
