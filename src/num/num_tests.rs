use crate::IncDecExt;

use core::num::{NonZeroI8, NonZeroU8};

#[test]
fn non_zero_i8_test()
{

    let mut int_val = NonZeroI8::new(1).unwrap();

    int_val.mm();

    assert_eq!(NonZeroI8::new(-1).unwrap(), int_val);

    //assert_eq!(NonZeroI8::new(i8::MAX).unwrap(), int_val);

    int_val.mm();

    assert_eq!(NonZeroI8::new(-2).unwrap(), int_val);

    //assert_eq!(NonZeroI8::new(i8::MAX - 1).unwrap(), int_val);

    int_val.pp();

    assert_eq!(NonZeroI8::new(-1).unwrap(), int_val);

    //assert_eq!(NonZeroI8::new(i8::MAX).unwrap(), int_val);

    int_val.pp();

    assert_eq!(NonZeroI8::new(1).unwrap(), int_val);

    int_val.pp();

    assert_eq!(NonZeroI8::new(2).unwrap(), int_val);

}

#[test]
fn non_zero_u8_test()
{

    let mut int_val = NonZeroU8::new(1).unwrap();

    //int_val.mm();

    int_val.pp();

    assert_eq!(NonZeroU8::new(2).unwrap(), int_val);

    //assert_eq!(NonZeroU8::new(u8::MAX).unwrap(), int_val);

    //assert_eq!(NonZeroI8::new(-1).unwrap(), int_val);

    /*
    assert_eq!(NonZeroU8::new(u8::MAX).unwrap(), int_val);

    int_val.mm();

    //assert_eq!(NonZeroI8::new(-2).unwrap(), int_val);

    assert_eq!(NonZeroU8::new(u8::MAX - 1).unwrap(), int_val);

    int_val.pp();

    //assert_eq!(NonZeroI8::new(-1).unwrap(), int_val);

    assert_eq!(NonZeroU8::new(u8::MAX).unwrap(), int_val);

    int_val.pp();

    assert_eq!(NonZeroU8::new(1).unwrap(), int_val);

    int_val.pp();

    assert_eq!(NonZeroU8::new(2).unwrap(), int_val);\
    */

}