
//Incrementation

///
/// Increments the provided integer by one.
/// 
#[macro_export]
macro_rules! pp
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
macro_rules! pp_mut
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
macro_rules! checked_pp_mut
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
#[macro_export]
macro_rules! ppf
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
macro_rules! ppf_mut
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
#[macro_export]
macro_rules! mm
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
macro_rules! mm_mut
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
#[macro_export]
macro_rules! mmf
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
macro_rules! mmf_mut
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
macro_rules! checked_mm_mut
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
macro_rules! opp_mut
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
macro_rules! omm_mut
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
macro_rules! wpp_mut
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
macro_rules! wmm_mut
{

    ($integer:ident) =>
    {

        {

            *$integer = $integer.wrapping_sub(1);

            *$integer

        }

    }

}
