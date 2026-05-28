
#[macro_export]
macro_rules! non_zero_opp_mut
{

    ($integer:ident, $integer_type:ty) =>
    {

        {

            let res: Result<$integer_type, <Self as TryInto<$integer_type>>::Error> = Self::try_into(*$integer);

            match res
            {

                Ok(val) =>
                {

                    let maybe_incd = val.overflowing_add(1);

                    if let Some(nz_incd) = Self::new(maybe_incd.0)
                    {

                        *$integer = nz_incd;

                        (*$integer, maybe_incd.1)

                    }
                    else
                    {

                        unsafe
                        {

                            *$integer = Self::new_unchecked(1);

                        }

                        (*$integer, false)
                        
                    }

                }
                Err(_err) =>
                {

                    unsafe
                    {

                        *$integer = Self::new_unchecked(1);

                    }

                    (*$integer, false)

                }

            }

        }

    }

}

#[macro_export]
macro_rules! non_zero_signed_omm_mut
{

    ($integer:ident, $integer_type:ty) =>
    {

        {

            let res: Result<$integer_type, <Self as TryInto<$integer_type>>::Error> = Self::try_into(*$integer);

            match res
            {

                Ok(val) =>
                {

                    let maybe_decd = val.overflowing_sub(1);

                    if let Some(nz_decd) = Self::new(maybe_decd.0)
                    {

                        *$integer = nz_decd;

                        (*$integer, maybe_decd.1)

                    }
                    else
                    {

                        unsafe
                        {

                            *$integer = Self::new_unchecked(-1);

                        }

                        (*$integer, false)
                        
                    }

                }
                Err(_err) =>
                {

                    unsafe
                    {

                        *$integer = Self::new_unchecked(-1);

                    }

                    (*$integer, false)

                }

            }

        }

    }

}

#[macro_export]
macro_rules! non_zero_unsigned_omm_mut
{

    ($integer:ident, $integer_type:ty) =>
    {

        {

            let res: Result<$integer_type, <Self as TryInto<$integer_type>>::Error> = Self::try_into(*$integer);

            match res
            {

                Ok(val) =>
                {

                    let maybe_decd = val.overflowing_sub(1);

                    if let Some(nz_decd) = Self::new(maybe_decd.0)
                    {

                        *$integer = nz_decd;

                        (*$integer, maybe_decd.1)

                    }
                    else
                    {

                        unsafe
                        {

                            *$integer = Self::new_unchecked($integer_type::MAX) //1);

                        }

                        (*$integer, true) //false)
                        
                    }

                }
                Err(_err) =>
                {

                    unsafe
                    {

                        *$integer = Self::new_unchecked(1);

                    }

                    (*$integer, false)

                }

            }

        }

    }

}

