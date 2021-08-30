#![cfg_attr(feature = "no_std", no_std)]
#![warn(missing_docs)]
//! This crate provides [`GetChecked`] and [`GetCheckedSliceIndex`] traits which provide
//! `get_checked` and `get_checked_mut` methods for [`array`] and [`slice`] types.
//!
//! These methods provide similar functionality as [`get`] and [`get_mut`] but return a
//! [`Result`] instead of an [`Option`]. This allows users to retrieve detailed error
//! information and handle errors in a more ergonomic way.
//!
//! # Examples:
//! Error details can be printed to provide context to the user.
//! ```
//! # use get_checked::GetChecked;
//! let v = [1, 2, 3];
//!
//! if let Err(e) = v.get_checked(1..4)
//! {
//!     println!("{}", e);
//!     assert_eq!(e.to_string(), "range end index 4 out of range for slice of length 3");
//! }
//! ```
//!
//! Error details can be extracted to provide custom error messages in external code.
//! ```
//! # use get_checked::{GetChecked, IndexErrorKind};
//! let v = [1, 2, 3];
//!
//! if let Err(e) = v.get_checked(4)
//! {
//!     match e.kind()
//!     {
//!         | IndexErrorKind::Bounds(index, len) => (/*..*/),
//!         | IndexErrorKind::Order(start, end) => (/*..*/),
//!         | IndexErrorKind::StartRange(start, len) => (/*..*/),
//!         | IndexErrorKind::EndRange(start, len) => (/*..*/),
//!         | IndexErrorKind::StartOverflow() => (/*..*/),
//!         | IndexErrorKind::EndOverflow() => (/*..*/),
//!         | _ => (/*..*/),
//!     };
//! }
//! ```
//! Error details can be wrapped using the [`From`] trait.
//! ```
//! use std::{error::Error, fmt};
//!
//! use get_checked::{GetChecked, IndexError};
//!
//! #[derive(Debug)]
//! struct MyError
//! {
//!     details: String,
//! }
//!
//! impl fmt::Display for MyError
//! {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
//!     {
//!         write!(f, "My Error: {}", self.details)
//!     }
//! }
//!
//! // Wrap error:
//! impl From<IndexError> for MyError
//! {
//!     fn from(err: IndexError) -> MyError
//!     {
//!         MyError { details: err.to_string() }
//!     }
//! }
//!
//! // Example usage:
//! fn foo() -> Result<(), MyError>
//! {
//!     let v = [1, 2, 3];
//!     v.get_checked(4)?;
//!     Ok(())
//! }
//!
//! fn main()
//! {
//!     assert_eq!(
//!         foo().unwrap_err().to_string(),
//!         "My Error: index out of bounds: the len is 4 but the index is 3"
//!     );
//! }
//! ```
//! [`get`]:              slice::get
//! [`get_mut`]:          slice::get_mut

use core::ops::{self, Bound, RangeBounds};

mod error;

pub use error::{IndexError, IndexErrorKind};

/// Type definition of [`IndexError`].
pub type Error = error::IndexError;
/// Type definition of [`IndexErrorKind`].
pub type ErrorKind = error::IndexErrorKind;

use error::IndexErrorKind::{Bounds, EndOverflow, EndRange, Order, StartOverflow, StartRange};

#[cfg(test)]
mod tests;

/// A helper trait used for adding [`get_checked`] and [`get_checked_mut`] indexing operations
/// to `usize`, `Range`, `RangeTo`, `RangeFrom`, `RangeFull`, `RangeInclusive`,
/// and `RangeToInclusive`.
///
/// [`get_checked`]:      GetCheckedSliceIndex::get_checked
/// [`get_checked_mut`]:  GetCheckedSliceIndex::get_checked_mut
pub trait GetCheckedSliceIndex<T: ?Sized>
{
    /// The output type returned by methods.
    type Output: ?Sized;

    /// Accepts a [`slice`] and returns a `Result` containing a reference to an element or
    /// subslice corresponding to the index upon which the method is called.
    ///
    /// - If called on a `usize`, returns an `Ok` containing a reference to the element
    ///   at that position or `Err` containing a `IndexError` describing the error if out of
    ///   bounds.
    /// - If called on a `range`, returns an `Ok` containing a reference to the subslice
    ///   corresponding to that range, or `Err` containing a `IndexError` describing the error
    ///   if out of bounds.
    ///
    /// # Errors
    ///
    /// If this function encounters any form of Error, an `IndexError`, containing a variant of
    /// type `IndexErrorKind` will be returned. Kind variants represent the type of index error
    /// encountered. These are retrievable via the [`kind`] method of the returned error.
    ///
    /// [`kind`]: IndexError::kind
    ///
    /// # Examples
    ///
    /// ```
    /// # use get_checked::GetCheckedSliceIndex;
    /// let v = [10, 40, 30];
    /// assert_eq!(Ok(&40), 1.get_checked(&v));
    /// assert_eq!(Ok(&[10, 40][..]), (0..2).get_checked(&v));
    ///
    /// if let Err(e) = 3.get_checked(&v)
    /// {
    ///     println!("Index error: {}", e);
    /// }
    ///
    /// if let Err(e) = (2..4).get_checked(&v)
    /// {
    ///     println!("Index error: {}", e);
    /// }
    /// ```
    fn get_checked(self, slice: &T) -> Result<&Self::Output, IndexError>;

    /// Accepts a mutable [`slice`] and returns a `Result` containing a mutable reference to an
    /// element or subslice corresponding to the index upon which the method is called.
    ///
    /// - If called on a `usize`, returns an `Ok` containing a mutable reference to the element
    ///   at that position or `Err` containing a `IndexError` describing the error if out of
    ///   bounds.
    /// - If called on a `range`, returns an `Ok` containing a reference to the mutable subslice
    ///   corresponding to that range, or `Err` containing a `IndexError` describing the error if
    ///   out of bounds.
    ///
    /// # Errors
    ///
    /// If this function encounters any form of Error, an `IndexError`, containing a variant of
    /// type [`IndexErrorKind`] will be returned. Kind variants represent the type of index error
    /// encountered. These are retrievable via the [`kind`] method of the returned error.
    ///
    /// [`kind`]: IndexError::kind
    ///
    /// # Examples
    ///
    /// ```
    /// # use get_checked::GetCheckedSliceIndex;
    /// let mut v = [0, 1, 2];
    /// if let Ok(elem) = 1.get_checked_mut(&mut v)
    /// {
    ///     *elem = 42;
    /// }
    ///
    /// if let Err(e) = 3.get_checked_mut(&mut v)
    /// {
    ///     println!("Index error: {}", e);
    /// }
    ///
    /// if let Err(e) = (2..4).get_checked_mut(&mut v)
    /// {
    ///     println!("Index error: {}", e);
    /// }
    /// ```
    fn get_checked_mut(self, slice: &mut T) -> Result<&mut Self::Output, IndexError>;
}

impl<T> GetCheckedSliceIndex<[T]> for usize
{
    type Output = T;

    #[inline] #[rustfmt::skip]
    fn get_checked(self, slice: &[T]) -> Result<&T, IndexError>
    {
        match self
        {
            | _ if self < slice.len() => unsafe { Ok(&*slice.get_unchecked(self)) },
            | _ => Err(Error { kind: Bounds(self, slice.len()) }),
        }
    }

    #[inline] #[rustfmt::skip]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut T, IndexError>
    {
        match self
        {
            | _ if self < slice.len() => unsafe { Ok(&mut *slice.get_unchecked_mut(self)) },
            | _ => Err(Error { kind: Bounds(self, slice.len()) }),
        }
    }
}

impl<T> GetCheckedSliceIndex<[T]> for ops::Range<usize>
{
    type Output = [T];

    #[inline] #[rustfmt::skip]
    fn get_checked(self, slice: &[T]) -> Result<&[T], IndexError>
    {
        let len = slice.len();
        match self
        {
            | _ if self.start > self.end => Err(Error { kind: Order(self.start, self.end) }),
            | _ if self.end > len => Err(Error { kind: EndRange(self.end, len) }),
            | _ => unsafe { Ok(&*slice.get_unchecked(self)) },
        }
    }

    #[inline] #[rustfmt::skip]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], IndexError>
    {
        let len = slice.len();
        match self
        {
            | _ if self.start > self.end => Err(Error { kind: Order(self.start, self.end) }),
            | _ if self.end > len => Err(Error { kind: EndRange(self.end, len) }),
            | _ => unsafe { Ok(&mut *slice.get_unchecked_mut(self)) },
        }
    }
}

impl<T> GetCheckedSliceIndex<[T]> for ops::RangeTo<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], IndexError>
    {
        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(Error { kind: EndOverflow() })?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if end > len => Err(Error { kind: EndRange(end, len) })?,
            | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], IndexError>
    {
        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(Error { kind: EndOverflow() })?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if end > len => Err(Error { kind: EndRange(end, len) })?,
            | _ => Ok(unsafe { &mut *slice.get_unchecked_mut(self) }),
        }
    }
}

impl<T> GetCheckedSliceIndex<[T]> for ops::RangeFrom<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], IndexError>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error { kind: StartOverflow() })?,
            | Bound::Unbounded => 0,
        };

        let len = slice.len();

        match slice
        {
            | _ if start > len => Err(Error { kind: StartRange(start, len) })?,
            | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], IndexError>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error { kind: StartOverflow() })?,
            | Bound::Unbounded => 0,
        };

        let len = slice.len();

        match slice
        {
            | _ if start > len => Err(Error { kind: StartRange(start, len) })?,
            | _ => Ok(unsafe { &mut *slice.get_unchecked_mut(self) }),
        }
    }
}

impl<T> GetCheckedSliceIndex<[T]> for ops::RangeFull
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], IndexError>
    {
        Ok(slice)
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], IndexError>
    {
        Ok(slice)
    }
}

impl<T> GetCheckedSliceIndex<[T]> for ops::RangeInclusive<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], IndexError>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error { kind: StartOverflow() })?,
            | Bound::Unbounded => 0,
        };

        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(Error { kind: EndOverflow() })?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if start > end => Err(Error { kind: Order(start, end) })?,
            | _ if end > len => Err(Error { kind: EndRange(end, len) })?,
            | _ => Ok(unsafe { &*slice.get_unchecked(self) }),
        }
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], IndexError>
    {
        let start = match self.start_bound()
        {
            | Bound::Included(x) => *x,
            | Bound::Excluded(x) => x.checked_add(1).ok_or(Error { kind: StartOverflow() })?,
            | Bound::Unbounded => 0,
        };

        let end = match self.end_bound()
        {
            | Bound::Included(x) => x.checked_add(1).ok_or(Error { kind: EndOverflow() })?,
            | Bound::Excluded(x) => *x,
            | Bound::Unbounded => slice.len(),
        };

        let len = slice.len();

        match slice
        {
            | _ if start > end => Err(Error { kind: Order(start, end) })?,
            | _ if end > len => Err(Error { kind: EndRange(end, len) })?,
            | _ => Ok(unsafe { &mut *slice.get_unchecked_mut(self) }),
        }
    }
}

impl<T> GetCheckedSliceIndex<[T]> for ops::RangeToInclusive<usize>
{
    type Output = [T];

    #[inline]
    fn get_checked(self, slice: &[T]) -> Result<&[T], IndexError>
    {
        (0..=self.end).get_checked(slice)
    }

    #[inline]
    fn get_checked_mut(self, slice: &mut [T]) -> Result<&mut [T], IndexError>
    {
        (0..=self.end).get_checked_mut(slice)
    }
}

/// Trait adding [`get_checked`] and [`get_checked_mut`] Indexing implementations to `[T]`.
///
/// [`get_checked`]: GetChecked::get_checked
/// [`get_checked_mut`]: GetChecked::get_checked_mut
pub trait GetChecked<T>
{
    /// Accepts a [`usize`] or [`range`] and returns a `Result` containing a reference to an
    /// element or subslice corresponding to the index.
    ///
    /// - If given a `usize`, returns an `Ok` containing a reference to the element
    ///   at that position or `Err` containing a `IndexError` describing the error if out of
    ///   bounds.
    /// - If given a `range`, returns an `Ok` containing a reference to the subslice
    ///   corresponding to that range, or `Err` containing a `IndexError` describing the error
    ///   if out of bounds.
    ///
    /// # Errors
    ///
    /// If this function encounters any form of Error, an `IndexError`, containing a variant of
    /// type [`IndexErrorKind`] will be returned. Kind variants represent the type of index error
    /// encountered. These are retrievable via the [`kind`] method of the returned error.
    ///
    /// [`range`]: ops::Range
    /// [`kind`]:  IndexError::kind
    ///
    /// # Examples
    ///
    /// ```
    /// # use get_checked::GetChecked;
    /// let v = [10, 40, 30];
    /// assert_eq!(Ok(&40), v.get_checked(1));
    /// assert_eq!(Ok(&[10, 40][..]), v.get_checked(0..2));
    ///
    /// if let Err(e) = v.get_checked(3)
    /// {
    ///     println!("Index error: {}", e);
    /// }
    ///
    /// if let Err(e) = v.get_checked(2..4)
    /// {
    ///     println!("Index error: {}", e);
    /// }
    /// ```
    #[inline]
    fn get_checked<I>(&self, index: I) -> Result<&I::Output, IndexError>
    where I: GetCheckedSliceIndex<Self>
    {
        index.get_checked(self)
    }

    /// Accepts a `usize` or `range` and returns a `Result` containing a mutable reference
    /// to an element or subslice subslice corresponding to the index.
    ///
    /// - If given a `usize`, returns an `Ok` containing a mutable reference to the element
    ///   at that position or `Err` containing a `IndexError` describing the error if out of
    ///   bounds.
    /// - If given a `range`, returns an `Ok` containing a reference to the mutable subslice
    ///   corresponding to that range, or `Err` containing a `IndexError` describing the error
    ///   if out of bounds.
    ///
    /// # Errors
    ///
    /// If this function encounters any form of Error, an `IndexError`, containing a variant of
    /// type [`IndexErrorKind`] will be returned. Kind variants represent the type of index error
    /// encountered. These are retrievable via the [`kind`] method of the returned error.
    ///
    /// [`range`]: ops::Range
    /// [`kind`]:  IndexError::kind
    ///
    /// # Examples
    ///
    /// ```
    /// # use get_checked::GetChecked;
    /// let v = &mut [0, 1, 2];
    /// if let Ok(elem) = v.get_checked_mut(1)
    /// {
    ///     *elem = 42;
    /// }
    ///
    /// if let Err(e) = v.get_checked_mut(3)
    /// {
    ///     println!("Index error: {}", e);
    /// }
    ///
    /// if let Err(e) = v.get_checked_mut(2..4)
    /// {
    ///     println!("Index error: {}", e);
    /// }
    /// ```
    #[inline]
    fn get_checked_mut<I>(&mut self, index: I) -> Result<&mut I::Output, IndexError>
    where I: GetCheckedSliceIndex<Self>
    {
        index.get_checked_mut(self)
    }
}

impl<T> GetChecked<T> for [T] {}
