use core::num::{NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize};

use crate::{IntIncDecExt, non_zero_signed_omm, non_zero_signed_opp, non_zero_signed_wmm, non_zero_unsigned_omm, non_zero_unsigned_opp, non_zero_unsigned_wmm, non_zero_wpp};


impl IntIncDecExt for NonZeroI8
{

    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_signed_opp!(self, i8)

        /*
        let res: Result<i8, <Self as TryInto<i8>>::Error> = Self::try_into(*self); //NonZeroI8::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let maybe_incd = val.overflowing_add(1); //val.pp();

                if let Some(nz_incd) = Self::new(maybe_incd.0)
                {

                   *self = nz_incd;

                   (*self, maybe_incd.1)

                }
                else
                {

                    unsafe
                    {

                        *self = Self::new_unchecked(1);

                    }

                    (*self, true)
                    
                }

            }
            Err(_err) =>
            {

                unsafe
                {

                    *self = Self::new_unchecked(1);

                }

                (*self, false)

            }

        }
        */

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_signed_omm!(self, i8)
        
        /*
        let res: Result<i8, <Self as TryInto<i8>>::Error> = Self::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let maybe_decd = val.overflowing_sub(1);

                if let Some(nz_decd) = Self::new(maybe_decd.0)
                {

                   *self = nz_decd;

                   (*self, maybe_decd.1)

                }
                else
                {

                    unsafe
                    {

                        *self = Self::new_unchecked(-1); //i8::MAX)

                    }

                    (*self, false) //true)
                    
                }

            }
            Err(_err) =>
            {

                unsafe
                {

                    *self = Self::new_unchecked(-1);

                }

                (*self, false)

            }

        }
        */

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, i8)

        /*
        let res: Result<i8, <Self as TryInto<i8>>::Error> = Self::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let maybe_incd = val.wrapping_add(1);

                if let Some(nz_incd) = Self::new(maybe_incd)
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

    fn wmm(&mut self) -> Self
    {

        non_zero_signed_wmm!(self, i8)

        /*
        let res: Result<i8, <Self as TryInto<i8>>::Error> = Self::try_into(*self);

        match res
        {

            Ok(val) =>
            {

                let maybe_decd = val.wrapping_sub(1);

                if let Some(nz_decd) = Self::new(maybe_decd)
                {

                   *self = nz_decd;

                   *self

                }
                else
                {

                    unsafe
                    {

                        *self = Self::new_unchecked(-1); //i8::MAX)

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

}

impl IntIncDecExt for NonZeroI16
{

    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_signed_opp!(self, i16)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_signed_omm!(self, i16)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, i16)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_signed_wmm!(self, i16)

    }

}


impl IntIncDecExt for NonZeroI32
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_signed_opp!(self, i32)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_signed_omm!(self, i32)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, i32)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_signed_wmm!(self, i32)

    }

}

impl IntIncDecExt for NonZeroI64
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_signed_opp!(self, i64)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_signed_omm!(self, i64)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, i64)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_signed_wmm!(self, i64)

    }

}

impl IntIncDecExt for NonZeroI128
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_signed_opp!(self, i128)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_signed_omm!(self, i128)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, i128)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_signed_wmm!(self, i128)

    }

}

impl IntIncDecExt for NonZeroIsize
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_signed_opp!(self, isize)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_signed_omm!(self, isize)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, isize)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_signed_wmm!(self, isize)

    }

}

impl IntIncDecExt for NonZeroU8
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_opp!(self, u8)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_omm!(self, u8)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, u8)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_unsigned_wmm!(self, u8)

    }

}

impl IntIncDecExt for NonZeroU16
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_opp!(self, u16)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_omm!(self, u16)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, u16)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_unsigned_wmm!(self, u16)

    }

}

impl IntIncDecExt for NonZeroU32
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_opp!(self, u32)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_omm!(self, u32)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, u32)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_unsigned_wmm!(self, u32)

    }

}

impl IntIncDecExt for NonZeroU64
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_opp!(self, u64)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_omm!(self, u64)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, u64)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_unsigned_wmm!(self, u64)

    }

}

impl IntIncDecExt for NonZeroU128
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_opp!(self, u128)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_omm!(self, u128)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, u128)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_unsigned_wmm!(self, u128)

    }

}

impl IntIncDecExt for NonZeroUsize
{
    
    fn opp(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_opp!(self, usize)

    }

    fn omm(&mut self) -> (Self, bool)
    {

        non_zero_unsigned_omm!(self, usize)

    }

    fn wpp(&mut self) -> Self
    {

        non_zero_wpp!(self, usize)

    }

    fn wmm(&mut self) -> Self
    {

        non_zero_unsigned_wmm!(self, usize)

    }

}