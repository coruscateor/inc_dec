
//Incrementation

///
/// Increments the provided integer by one.
/// 
/// ```
///
///    use inc_dec::pp_;
///
///    let mut val = 1;
///
///    pp_!(val);
///
///    assert_eq!(2, val);
///
///```
/// 
#[macro_export]
macro_rules! pp_
{

    ($integer:ident) =>
    {

        $integer += 1;

    }

}

///
/// Increments the provided integer by one and returns it.
/// 
#[macro_export]
macro_rules! pp
{

    ($integer:ident) =>
    {

        {

            *$integer += 1;

            *$integer

        }

    }

}

///
/// Calls checked_add on an integer with 1 as a parameter.
/// 
/// Overwrites the provided integer.
/// 
#[macro_export]
macro_rules! try_pp
{

    ($integer:ident) =>
    {

        if let Some(res) = $integer.checked_add(1)
        {

            *$integer = res;

            Some(*$integer)

        }
        else
        {

            None
            
        }

    }

}

///
/// Increments the provided floating point number by one.
/// 
///```
///
///    use inc_dec::ppf_;
///
///    let mut val: f32 = 1.0;
///
///    ppf_!(val);
///
///    assert_eq!(2.0, val);
///
///    let mut val = 1.0;
///
///    ppf_!(val);
///
///    assert_eq!(2.0, val);
///
///```
#[macro_export]
macro_rules! ppf_
{

    ($float:ident) =>
    {

        $float += 1.0;

    }

}

///
/// Increments the provided floating point number by one and returns it.
/// 
#[macro_export]
macro_rules! ppf
{

    ($float:ident) =>
    {

        {

            *$float += 1.0;

            *$float

        }

    }

}

/*
macro expansion ignores token `*` and any following
the usage of `ppf_mut!` is likely invalid in expression contextrustcClick for full compiler diagnostic
inc_dec.rs(205, 9): caused by the macro expansion here
inc_dec.rs(205, 23): you might be missing a semicolon here: `;`
No quick fixes available
*/

//Decrementation

///
/// Decrements the provided integer by one.
/// 
/// ```
///    use inc_dec::mm_;
///
///    let mut val = 2;
///
///    mm_!(val);
///
///    assert_eq!(1, val);
/// 
/// ```
#[macro_export]
macro_rules! mm_
{

    ($integer:ident) =>
    {

        $integer -= 1;

    }

}

///
/// Decrements the provided integer by one and returns it.
/// 
#[macro_export]
macro_rules! mm
{

    ($integer:ident) =>
    {

        {

            *$integer -= 1;

            *$integer

        }

    }

}

///
/// Decrements the provided floating point number by one.
/// 
///
///```
///
///    use inc_dec::mmf_;
///
///    let mut val: f32 = 2.0;
///
///    mmf_!(val);
///
///    assert_eq!(1.0, val);
///
///    let mut val = 2.0;
///
///    mmf_!(val);
///
///    assert_eq!(1.0, val);
///
///```
#[macro_export]
macro_rules! mmf_
{

    ($float:ident) =>
    {

        $float -= 1.0;

    }

}

///
/// Decrements the provided floating point number by one and returns it.
/// 
#[macro_export]
macro_rules! mmf
{

    ($float:ident) =>
    {

        {

            *$float -= 1.0;

            *$float

        }

    }

}

///
/// Calls checked_sub on an integer with 1 as a parameter.
/// 
/// Overwrites the provided integer.
/// 
#[macro_export]
macro_rules! try_mm
{

    ($integer:ident) =>
    {

        if let Some(res) = $integer.checked_sub(1)
        {

            *$integer = res;

            Some(*$integer)

        }
        else
        {

            None
            
        }

    }

}

//Integers Only

///
/// Calls overflowing_add on an integer with 1 as a parameter.
///
/// Overwites the provided integer with the first value and returns the tuple.
/// 
#[macro_export]
macro_rules! opp
{

    ($integer:ident) =>
    {

        {

            let res = $integer.overflowing_add(1);

            *$integer = res.0;

            res

        }

    }

}

///
/// Calls overflowing_sub on an integer with 1 as a parameter.
/// 
/// Overwites the provided integer with the first value and returns the tuple.
/// 
#[macro_export]
macro_rules! omm
{

    ($integer:ident) =>
    {

        {

            let res = $integer.overflowing_sub(1);

            *$integer = res.0;

            res

        }

    }

}

///
/// Calls wrapping_add on an integer with 1 as a parameter.
///
/// Overwites the provided integer and returns it.
/// 
#[macro_export]
macro_rules! wpp
{

    ($integer:ident) =>
    {

        {

            *$integer = $integer.wrapping_add(1);

            *$integer

        }

    }

}

///
/// Calls wrapping_sub on an integer with 1 as a parameter.
///
/// Overwites the provided integer and returns it.
/// 
#[macro_export]
macro_rules! wmm
{

    ($integer:ident) =>
    {

        {

            *$integer = $integer.wrapping_sub(1);

            *$integer

        }

    }

}
