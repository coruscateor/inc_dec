use crate::{IncDecExt, IntIncDecExt};

use core::num::{NonZeroI8, NonZeroU8};

#[test]
fn non_zero_i8_test()
{

    let mut int_val = NonZeroI8::new(1).unwrap();

    assert_eq!(Some(NonZeroI8::new(-1).unwrap()), int_val.try_mm());

    assert_eq!(Some(NonZeroI8::new(-2).unwrap()), int_val.try_mm());

    assert_eq!(Some(NonZeroI8::new(-1).unwrap()), int_val.try_pp());

    assert_eq!(Some(NonZeroI8::new(1).unwrap()), int_val.try_pp());

    assert_eq!(Some(NonZeroI8::new(2).unwrap()), int_val.try_pp());

}

#[test]
fn non_zero_u8_test()
{

    let mut int_val = NonZeroU8::new(1).unwrap();

    assert_eq!(None, int_val.try_mm());

    //assert_eq!(Some(NonZeroI8::new(-2).unwrap()), int_val.try_mm());

    //assert_eq!(Some(NonZeroI8::new(-1).unwrap()), int_val.try_pp());

    //assert_eq!(Some(NonZeroU8::new(1).unwrap()), int_val.try_pp());

    assert_eq!(Some(NonZeroU8::new(2).unwrap()), int_val.try_pp());

}