
use crate::{IncDecExt, checked_mm_mut, checked_pp_mut, mm_mut, mmf_mut, pp_mut, ppf_mut};

//Integers and floating-point intergers.

//f32

impl IncDecExt for f32
{

    fn pp(&mut self) -> Self
    {

        ppf_mut!(self) //;

        //*self

        /*
        let mut val = *self;

        ppf!(val);

        *self = val;

        val
        */

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        if *self < (f32::MAX - 0.9)
        {

            Some(ppf_mut!(self)) //;

            //Some(*self)

        }
        else
        {

            None
            
        }

    }

    fn mm(&mut self) -> Self
    {

        mmf_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        if *self > (f32::MIN + 0.9)
        {

            Some(mmf_mut!(self)) //;

            //Some(*self)

        }
        else
        {

            None
            
        }

    }

}

//f64

impl IncDecExt for f64
{

    fn pp(&mut self) -> Self
    {

        ppf_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        if *self < (f64::MAX - 0.9)
        {

            Some(ppf_mut!(self)) //;

            //Some(*self)

        }
        else
        {

            None
            
        }

    }

    fn mm(&mut self) -> Self
    {

        mmf_mut!(self)  //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        if *self > (f64::MIN + 0.9)
        {

            Some(mmf_mut!(self)) //;

            //Some(*self)

        }
        else
        {

            None
            
        }
        
    }

}

//i8

impl IncDecExt for i8
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

        /*
        if let Some(res) = self.checked_add(1)
        {

            *self = res;

            Some(*self)

        }
        else
        {

            None
            
        }
        */

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i16

impl IncDecExt for i16
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }


}

//i32

impl IncDecExt for i32
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i64

impl IncDecExt for i64
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i128

impl IncDecExt for i128
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//isize

impl IncDecExt for isize
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u8

impl IncDecExt for u8
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u16

impl IncDecExt for u16
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u32

impl IncDecExt for u32
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u64

impl IncDecExt for u64
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u128

impl IncDecExt for u128
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//usize

impl IncDecExt for usize
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}
