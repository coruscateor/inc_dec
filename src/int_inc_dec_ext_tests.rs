use crate::IntIncDecExt;

#[test]
fn opp_omm_test_1()
{

    let mut val: u32 = 0;

    assert_eq!((1, false), val.opp());

    assert_eq!((0, false), val.omm());

    assert_eq!((u32::MAX, true), val.omm());

}

#[test]
fn opp_omm_test_2()
{

    let mut val: u32 = u32::MAX;

    assert_eq!((0, true), val.opp());

    assert_eq!((1, false), val.opp());

    assert_eq!((0, false), val.omm());

    assert_eq!((u32::MAX, true), val.omm());

}

#[test]
fn wpp_wmm_test_1()
{

    let mut val: u32 = 0;

    assert_eq!(1, val.wpp());

    assert_eq!(0, val.wmm());

    assert_eq!(u32::MAX, val.wmm());

}

#[test]
fn wpp_wmm_test_2()
{

    let mut val: u32 = u32::MAX;

    assert_eq!(0, val.wpp());

    assert_eq!(1, val.wpp());

    assert_eq!(0, val.wmm());

    assert_eq!(u32::MAX, val.wmm());

}
