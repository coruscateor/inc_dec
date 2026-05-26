
use crate::{IncDecSelf, IntIncDecSelf, checked_mm_mut, checked_pp_mut, mm_mut, mmf_mut, omm_mut, opp_mut, pp_mut, ppf_mut, wmm_mut, wpp_mut};



//Integers and floating-point intergers.

//f32

impl IncDecSelf for f32
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

impl IncDecSelf for f64
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

impl IncDecSelf for i8
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

impl IncDecSelf for i16
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

impl IncDecSelf for i32
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

impl IncDecSelf for i64
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

impl IncDecSelf for i128
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

impl IncDecSelf for isize
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

impl IncDecSelf for u8
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

impl IncDecSelf for u16
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

impl IncDecSelf for u32
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

impl IncDecSelf for u64
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

impl IncDecSelf for u128
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

impl IncDecSelf for usize
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

//Integers Only

//i8

impl IntIncDecSelf for i8
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)

        /*
        let res = self.overflowing_add(1);

        *self = res.0;

        res
        */
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

        /*
        let res =self.overflowing_sub(1);

        *self = res.0;

        res
        */

    }
    
    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

        /*
        *self = self.wrapping_add(1);

        *self
        */

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

        /*
        *self = self.wrapping_sub(1);

        *self
        */

    }

}

//i16

impl IntIncDecSelf for i16
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//i32

impl IntIncDecSelf for i32
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//i64

impl IntIncDecSelf for i64
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//i128

impl IntIncDecSelf for i128
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//isize

impl IntIncDecSelf for isize
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//u8

impl IntIncDecSelf for u8
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//u16

impl IntIncDecSelf for u16
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//u32

impl IntIncDecSelf for u32
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//u64

impl IntIncDecSelf for u64
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//u128

impl IntIncDecSelf for u128
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}

//usize

impl IntIncDecSelf for usize
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp_mut!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm_mut!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp_mut!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm_mut!(self)

    }

}
