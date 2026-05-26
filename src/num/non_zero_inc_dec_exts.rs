use core::num::NonZeroI8;

use crate::IncDecExt;


impl IncDecExt for NonZeroI8
{

    fn pp(&mut self) -> Self
    {

        //let res: Result<i8, TryFrom<NonZero<i8>>>::Error > = (*self).try_into(); //: Result<i8, NonZero<i8>> //i8 as TryInto<i8>::Error //<i8 as TryFrom::Error>

        let res: Result<i8, <Self as TryInto<i8>>::Error> = NonZeroI8::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let incd = val + 1; //val.pp();

                if let Some(nz_incd) = Self::new(incd)
                {

                   *self = nz_incd;

                   *self

                }
                else
                {

                    unsafe
                    {

                        *self = Self::new_unchecked(1);

                    }

                    *self
                    
                }

            }
            Err(_err) =>
            {

                unsafe
                {

                    *self = Self::new_unchecked(1);

                }

                *self

            }

        }

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        let res: Result<i8, <Self as TryInto<i8>>::Error> = NonZeroI8::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let opt_incd = val.checked_add(1); //val.pp();

                if let Some(incd) = opt_incd
                {

                    if let Some(nz_incd) = Self::new(incd)
                    {

                        *self = nz_incd;

                        Some(*self)

                    }
                    else
                    {

                        unsafe
                        {

                            *self = Self::new_unchecked(1);

                        }

                        Some(*self)
                        
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

                    *self = Self::new_unchecked(1);

                }

                Some(*self)

            }
            
        }
        
    }

    fn mm(&mut self) -> Self
    {

        let res: Result<i8, <Self as TryInto<i8>>::Error> = NonZeroI8::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let incd = val - 1; //val.pp();

                if let Some(nz_incd) = Self::new(incd)
                {

                   *self = nz_incd;

                   *self

                }
                else
                {

                    unsafe
                    {

                        *self = Self::new_unchecked(-1);

                    }

                    *self
                    
                }

            }
            Err(_err) =>
            {

                unsafe
                {

                    *self = Self::new_unchecked(-1);

                }

                *self

            }

        }
    }

    fn try_mm(&mut self) -> Option<Self>
    {

        let res: Result<i8, <Self as TryInto<i8>>::Error> = NonZeroI8::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let opt_incd = val.checked_sub(1); //val.pp();

                if let Some(incd) = opt_incd
                {

                    if let Some(nz_incd) = Self::new(incd)
                    {

                        *self = nz_incd;

                        Some(*self)

                    }
                    else
                    {

                        unsafe
                        {

                            *self = Self::new_unchecked(-1);

                        }

                        Some(*self)
                        
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

                    *self = Self::new_unchecked(-1);

                }

                Some(*self)

            }
            
        }

    }

}