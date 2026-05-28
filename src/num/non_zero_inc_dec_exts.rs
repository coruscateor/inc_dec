use core::num::{NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128,NonZeroIsize,  NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize};

use crate::{IncDecExt, non_zero_pp_mut, non_zero_signed_mm_mut, non_zero_signed_try_mm_mut, non_zero_try_pp_mut, non_zero_unsigned_mm_mut, non_zero_unsigned_try_mm_mut};


impl IncDecExt for NonZeroI8
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, i8) //, Self)

        //let res: Result<i8, TryFrom<NonZero<i8>>>::Error > = (*self).try_into(); //: Result<i8, NonZero<i8>> //i8 as TryInto<i8>::Error //<i8 as TryFrom::Error>

        /*
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
        */

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, i8) //, Self)

        /*
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
        */
        
    }

    fn mm(&mut self) -> Self
    {

        non_zero_signed_mm_mut!(self, i8) //, Self)

        /*
        let res: Result<i8, <Self as TryInto<i8>>::Error> = NonZeroI8::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let decd = val - 1; //val.pp();

                if let Some(nz_decd) = Self::new(decd)
                {

                   *self = nz_decd;

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
        */

    }

    fn try_mm(&mut self) -> Option<Self>
    {

        non_zero_signed_try_mm_mut!(self, i8) //, Self)

        /*
        let res: Result<i8, <Self as TryInto<i8>>::Error> = NonZeroI8::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let opt_decd = val.checked_sub(1); //val.pp();

                if let Some(decd) = opt_decd
                {

                    if let Some(nz_decd) = Self::new(decd)
                    {

                        *self = nz_decd;

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
        */

    }
    
}

impl IncDecExt for NonZeroI16
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, i16) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, i16) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_signed_mm_mut!(self, i16) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

        non_zero_signed_try_mm_mut!(self, i16) //, Self)

    }

}

impl IncDecExt for NonZeroI32
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, i32) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, i32) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_signed_mm_mut!(self, i32) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

        non_zero_signed_try_mm_mut!(self, i32) //, Self)

    }

}

impl IncDecExt for NonZeroI64
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, i64) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, i64) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_signed_mm_mut!(self, i64) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

        non_zero_signed_try_mm_mut!(self, i64) //, Self)

    }


}

impl IncDecExt for NonZeroI128
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, i128) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, i128) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_signed_mm_mut!(self, i128) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

        non_zero_signed_try_mm_mut!(self, i128) //, Self)

    }

}

impl IncDecExt for NonZeroIsize
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, isize) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, isize) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_signed_mm_mut!(self, isize) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

        non_zero_signed_try_mm_mut!(self, isize) //, Self)

    }

}

impl IncDecExt for NonZeroU8
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, u8) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, u8) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_unsigned_mm_mut!(self, u8) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

       non_zero_unsigned_try_mm_mut!(self, u8) //, Self)

    }

}

impl IncDecExt for NonZeroU16
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, u16) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, u16) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_unsigned_mm_mut!(self, u16) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

       non_zero_unsigned_try_mm_mut!(self, u16) //, Self)

    }

}

impl IncDecExt for NonZeroU32
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, u32) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, u32) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_unsigned_mm_mut!(self, u32) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

       non_zero_unsigned_try_mm_mut!(self, u32) //, Self)

    }

}

impl IncDecExt for NonZeroU64
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, u64) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, u64) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_unsigned_mm_mut!(self, u64) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

       non_zero_unsigned_try_mm_mut!(self, u64) //, Self)

    }

}

impl IncDecExt for NonZeroU128
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, u128) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, u128) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_unsigned_mm_mut!(self, u128) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

       non_zero_unsigned_try_mm_mut!(self, u128) //, Self)

    }

}


impl IncDecExt for NonZeroUsize
{

    fn pp(&mut self) -> Self
    {

        non_zero_pp_mut!(self, usize) //, Self)

    }

    fn try_pp(&mut self) -> Option<Self>
    {

        non_zero_try_pp_mut!(self, usize) //, Self)
    }

    fn mm(&mut self) -> Self
    {

        non_zero_unsigned_mm_mut!(self, usize) //, Self)

    }

    fn try_mm(&mut self) -> Option<Self>
    {

       non_zero_unsigned_try_mm_mut!(self, usize) //, Self)

    }

}
