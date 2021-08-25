#[cfg(test)]
use super::{Error, GetChecked};

// Immutable tests:

#[test]
fn immut_index()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = *bytes.get_checked(4).unwrap();
    assert_eq!(ret, bytes[4]);
}

#[test]
fn immut_index_edge()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = *bytes.get_checked(15).unwrap();
    assert_eq!(ret, bytes[15]);
}

#[test]
fn immut_index_error()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];
    let err = bytes.get_checked(16).unwrap_err();

    assert_eq!(err.to_string(), "index out of bounds: the len is 16 but the index is 16");
}

#[test]
fn immut_range_full()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(..).unwrap();
    assert_eq!(ret.len(), 16);
    assert_eq!(ret, bytes);
}

#[test]
fn immut_range_full_exclusive()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(2..5).unwrap();
    assert_eq!(ret.len(), 3);
    assert_eq!(ret, &bytes[2..5]);
}

#[test]
fn immut_range_full_exclusive_edge()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(2..16).unwrap();
    assert_eq!(ret.len(), 14);
    assert_eq!(ret, &bytes[2..16]);
}

#[test]
fn immut_range_full_inclusive()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(2..=5).unwrap();
    assert_eq!(ret.len(), 4);
    assert_eq!(ret, &bytes[2..=5]);
}

#[test]
fn immut_range_full_inclusive_edge()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(2..=15).unwrap();
    assert_eq!(ret.len(), 14);
    assert_eq!(ret, &bytes[2..=15]);
}

#[test]
fn immut_range_full_zero_exclusive()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(0..0).unwrap();
    assert_eq!(ret.len(), 0);
    assert_eq!(ret, &bytes[0..0]);
}

#[test]
fn immut_range_full_zero_inclusive()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(0..=0).unwrap();
    assert_eq!(ret.len(), 1);
    assert_eq!(ret, &bytes[0..=0]);
}

#[test]
fn immut_range_from()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(5..).unwrap();
    assert_eq!(ret.len(), 16 - 5);
    assert_eq!(ret, &bytes[5..]);
}

#[test]
fn immut_range_from_zero()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(16..).unwrap();
    assert_eq!(ret.len(), 0);
    assert_eq!(ret, &bytes[16..]);
}

#[test]
fn immut_range_to_exclusive()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(..5).unwrap();
    assert_eq!(ret.len(), 5);
    assert_eq!(ret, &bytes[..5]);
}

#[test]
fn immut_range_to_inclusive()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let ret = bytes.get_checked(..=5).unwrap();
    assert_eq!(ret.len(), 6);
    assert_eq!(ret, &bytes[..=5]);
}

#[test]
fn immut_range_from_error()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked(17..).unwrap_err();
    assert_eq!(err.to_string(), "range start index 17 out of range for slice of length 16");
}

#[test]
fn immut_range_to_error()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked(..17).unwrap_err();
    assert_eq!(err.to_string(), "range end index 17 out of range for slice of length 16");
}

#[test]
fn immut_range_to_inclusive_error()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked(..=16).unwrap_err();
    assert_eq!(err.to_string(), "range end index 17 out of range for slice of length 16");
}

#[test]
fn immut_range_from_slice_error()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked(17..5).unwrap_err();
    assert_eq!(err.to_string(), "slice index starts at 17 but ends at 5");
}

#[test]
fn immut_range_slice_inclusive_error()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked(17..=4).unwrap_err();
    assert_eq!(err.to_string(), "slice index starts at 17 but ends at 5");
}

#[test]
fn immut_range_overflow_error()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked(0..=usize::MAX).unwrap_err();
    assert_eq!(err.to_string(), Error::EndIndexOverflowError().to_string());
}

// Mutable tests:

#[test]
fn mut_index()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];
    let mut bytes2 = bytes.clone();

    let ret = *bytes2.get_checked_mut(4).unwrap();
    assert_eq!(ret, bytes[4]);
}

#[test]
fn mut_index_edge()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];
    let mut bytes2 = bytes.clone();

    let ret = *bytes2.get_checked_mut(15).unwrap();
    assert_eq!(ret, bytes[15]);
}

#[test]
fn mut_index_error()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];
    let mut bytes2 = bytes.clone();

    let err = bytes2.get_checked_mut(16).unwrap_err();
    assert_eq!(err.to_string(), "index out of bounds: the len is 16 but the index is 16");
}

#[test]
fn mut_range_full()
{
    let bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
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
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked_mut(17..).unwrap_err();
    assert_eq!(err.to_string(), "range start index 17 out of range for slice of length 16");
}

#[test]
fn mut_range_to_error()
{
    let mut bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked_mut(..17).unwrap_err();
    assert_eq!(err.to_string(), "range end index 17 out of range for slice of length 16");
}

#[test]
fn mut_range_to_inclusive_error()
{
    let mut bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked_mut(..=16).unwrap_err();
    assert_eq!(err.to_string(), "range end index 17 out of range for slice of length 16");
}

#[test]
fn mut_range_slice_error()
{
    let mut bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked_mut(17..5).unwrap_err();
    assert_eq!(err.to_string(), "slice index starts at 17 but ends at 5");
}

#[test]
fn mut_range_slice_inclusive_error()
{
    let mut bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked_mut(17..=4).unwrap_err();
    assert_eq!(err.to_string(), "slice index starts at 17 but ends at 5");
}

#[test]
fn mut_range_overflow_error()
{
    let mut bytes = vec![
        0xA0, 0x11, 0xB2, 0xD3, 0x0F4, 0x35, 0x66, 0x17, 0x53, 0x65, 0xDA, 0xCB, 0x4C, 0xD5, 0x3E,
        0x1F,
    ];

    let err = bytes.get_checked_mut(0..=usize::MAX).unwrap_err();
    assert_eq!(err.to_string(), Error::EndIndexOverflowError().to_string());
}
