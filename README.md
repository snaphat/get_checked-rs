# get_checked-rs
Implementation of get_checked() and get_checked_mut() for Rust that return Result&lt;>

[![Rust](https://github.com/snaphat/get_checked-rs/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/snaphat/get_checked-rs/actions/workflows/rust.yml)
---

This crate provides `GetChecked` and `GetCheckedSliceIndex` traits which provide
`get_checked` and `get_checked_mut` methods for `array` and `slice` types.

These methods provide similar functionality as `get` and `get_mut` but return a
`Result` instead of an `Option`. This allows users to retrieve detailed error
information and handle errors in a more ergonomic way.

# Examples
Error details can be printed to provide context to the user.
```rust
# use get_checked::GetChecked;
let v = [1, 2, 3];

if let Err(e) = v.get_checked(1..4)
{
    println!("{}", e);
    assert_eq!(e.to_string(), "range end index 4 out of range for slice of length 3");
}
```

Error details can be extracted to provide custom error messages in external code.
```rust
# use get_checked::{GetChecked, IndexErrorKind};
let v = [1, 2, 3];

if let Err(e) = v.get_checked(4)
{
    match e.kind()
    {
        | IndexErrorKind::Bounds(index, len) => (/*..*/),
        | IndexErrorKind::Order(start, end) => (/*..*/),
        | IndexErrorKind::StartRange(start, len) => (/*..*/),
        | IndexErrorKind::EndRange(start, len) => (/*..*/),
        | IndexErrorKind::StartOverflow() => (/*..*/),
        | IndexErrorKind::EndOverflow() => (/*..*/),
        | _ => (/*..*/),
    };
}
```
Error details can be wrapped using the `From` trait.
```rust
use std::{error::Error, fmt};

use get_checked::{GetChecked, IndexError};

#[derive(Debug)]
struct MyError
{
    details: String,
}

impl fmt::Display for MyError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "My Error: {}", self.details)
    }
}

// Wrap error:
impl From<IndexError> for MyError
{
    fn from(err: IndexError) -> MyError
    {
        MyError { details: err.to_string() }
    }
}

// Example usage:
fn foo() -> Result<(), MyError>
{
    let v = [1, 2, 3];
    v.get_checked(4)?;
    Ok(())
}

fn main()
{
    assert_eq!(
        foo().unwrap_err().to_string(),
        "My Error: index out of bounds: the len is 4 but the index is 3"
    );
}
```
