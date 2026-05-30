
#[macro_export]
macro_rules! non_zero_pp_mut
{

    ($integer:ident, $integer_type:ty) => //, $self_integer_type:ty) =>
    {

        {

            let res: Result<$integer_type, <Self as TryInto<$integer_type>>::Error> = Self::try_into(*$integer); //<$self_integer_type>

            match res
            {

                Ok(val) =>
                {

                    let incd = val + 1; //val.pp();

                    if let Some(nz_incd) = Self::new(incd)
                    {

                        *$integer = nz_incd;

                        *$integer

                    }
                    else
                    {

                        unsafe
                        {

                            *$integer = Self::new_unchecked(1);

                        }

                        *$integer
                        
                    }

                }
                Err(_err) =>
                {

                    unsafe
                    {

                        *$integer = Self::new_unchecked(1);

                    }

                    *$integer

                }

            }

        }

    }

}

#[macro_export]
macro_rules! non_zero_try_pp_mut
{

    ($integer:ident, $integer_type:ty) => //, $self_integer_type:ty) =>
    {

        {

            let res: Result<$integer_type, <Self as TryInto<$integer_type>>::Error> = Self::try_into(*$integer); //<$self_integer_type>

            match res
            {

                Ok(val) =>
                {

                    let opt_incd = val.checked_add(1); //val.pp();

                    if let Some(incd) = opt_incd
                    {

                        if let Some(nz_incd) = Self::new(incd)
                        {

                            *$integer = nz_incd;

                            Some(*$integer)

                        }
                        else
                        {

                            unsafe
                            {

                                *$integer = Self::new_unchecked(1);

                            }

                            Some(*$integer)
                            
                        }

                    }
                    else
                    {
                        
                        None

                    }

                }
                Err(_err) =>
                {

                    unsafe
                    {

                        *$integer = Self::new_unchecked(1);

                    }

                    Some(*$integer)

                }
                
            }

        }

    }

}

//signed

#[macro_export]
macro_rules! non_zero_signed_mm_mut
{

    ($integer:ident, $integer_type:ty) => //, $self_integer_type:ty) =>
    {

        {

            let res: Result<$integer_type, <Self as TryInto<$integer_type>>::Error> = Self::try_into(*$integer); //<$self_integer_type>

            match res
            {

                Ok(val) =>
                {

                    let decd = val - 1; //val.pp();

                    if let Some(nz_decd) = Self::new(decd)
                    {

                        *$integer = nz_decd;

                        *$integer

                    }
                    else
                    {

                        unsafe
                        {

                            *$integer = Self::new_unchecked(-1);

                        }

                        *$integer
                        
                    }

                }
                Err(_err) =>
                {

                    unsafe
                    {

                        *$integer = Self::new_unchecked(-1);

                    }

                    *$integer

                }

            }

        }

    }

}

#[macro_export]
macro_rules! non_zero_signed_try_mm_mut
{

    ($integer:ident, $integer_type:ty) => //, $self_integer_type:ty) =>
    {

        {

            let res: Result<$integer_type, <Self as TryInto<$integer_type>>::Error> = Self::try_into(*$integer); //<$self_integer_type>

            match res
            {

                Ok(val) =>
                {

                    let opt_decd = val.checked_sub(1); //val.pp();

                    if let Some(decd) = opt_decd
                    {

                        if let Some(nz_decd) = Self::new(decd)
                        {

                            *$integer = nz_decd;

                            Some(*$integer)

                        }
                        else
                        {

                            unsafe
                            {

                                *$integer = Self::new_unchecked(-1);

                            }

                            Some(*$integer)
                            
                        }

                    }
                    else
                    {
                        
                        None

                    }

                }
                Err(_err) =>
                {

                    unsafe
                    {

                        *$integer = Self::new_unchecked(-1);

                    }

                    Some(*$integer)

                }

            }

        }

    }

}

//unsigned

#[macro_export]
macro_rules! non_zero_unsigned_mm_mut
{

    ($integer:ident, $integer_type:ty) => //, $self_integer_type:ty) =>
    {

        {

            let res: Result<$integer_type, <Self as TryInto<$integer_type>>::Error> = Self::try_into(*$integer); //<$self_integer_type>

            match res
            {

                Ok(val) =>
                {

                    let decd = val - 1; //val.pp();

                    if let Some(nz_decd) = Self::new(decd)
                    {

                        *$integer = nz_decd;

                        *$integer

                    }
                    else
                    {

                        panic!("Error: Value cannot be zero.")
                        
                        //Will it wrap?

                        /*
                        let val = 0 - 1;

                        unsafe
                        {

                            *$integer = Self::new_unchecked(val);

                        }

                        *$integer
                        */
                        
                    }

                }
                Err(_err) =>
                {

                    unsafe
                    {

                        *$integer = Self::new_unchecked(1);

                    }

                    *$integer

                }

            }

        }

    }

}

#[macro_export]
macro_rules! non_zero_unsigned_try_mm_mut
{

    ($integer:ident, $integer_type:ty) => //, $self_integer_type:ty) =>
    {

        {

            let res: Result<$integer_type, <Self as TryInto<$integer_type>>::Error> = Self::try_into(*$integer);

            match res
            {

                Ok(val) =>
                {

                    let opt_decd = val.checked_sub(1); //val.pp();

                    if let Some(decd) = opt_decd
                    {

                        if let Some(nz_decd) = Self::new(decd)
                        {

                            *$integer = nz_decd;

                            Some(*$integer)

                        }
                        else
                        {

                            None

                            /*
                            unsafe
                            {

                                *$integer = Self::new_unchecked(1);

                            }

                            Some(*$integer)
                            */
                            
                        }

                    }
                    else
                    {
                        
                        None

                    }

                }
                Err(_err) =>
                {

                    unsafe
                    {

                        *$integer = Self::new_unchecked(1);

                    }

                    Some(*$integer)

                }

            }

        }

    }

}

