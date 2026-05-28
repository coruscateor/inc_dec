use core::num::NonZeroI8;

use crate::IntIncDecExt;


impl IntIncDecExt for NonZeroI8
{

    fn opp(&mut self) -> (Self, bool)
    {

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

    }

    fn omm(&mut self) -> (Self, bool)
    {

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

                        *self = Self::new_unchecked(i8::MAX) //-1);

                    }

                    (*self, true)
                    
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

    }

    fn wpp(&mut self) -> Self
    {
        
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

    }

    fn wmm(&mut self) -> Self
    {

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

}
