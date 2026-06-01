use crate::{IntIncDecExt, omm, opp, wmm, wpp};

//Integers Only

//i8

impl IntIncDecExt for i8
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)

        /*
        let res = self.overflowing_add(1);

        *self = res.0;

        res
        */
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

        /*
        let res =self.overflowing_sub(1);

        *self = res.0;

        res
        */

    }
    
    fn wpp(&mut self) -> Self
    {

        wpp!(self)

        /*
        *self = self.wrapping_add(1);

        *self
        */

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

        /*
        *self = self.wrapping_sub(1);

        *self
        */

    }

}

//i16

impl IntIncDecExt for i16
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//i32

impl IntIncDecExt for i32
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//i64

impl IntIncDecExt for i64
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//i128

impl IntIncDecExt for i128
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//isize

impl IntIncDecExt for isize
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//u8

impl IntIncDecExt for u8
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//u16

impl IntIncDecExt for u16
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//u32

impl IntIncDecExt for u32
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//u64

impl IntIncDecExt for u64
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//u128

impl IntIncDecExt for u128
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}

//usize

impl IntIncDecExt for usize
{

    fn opp(&mut self) -> (Self, bool)
    {

        opp!(self)
        
    }

    fn omm(&mut self) -> (Self, bool)
    {

        omm!(self)

    }

    fn wpp(&mut self) -> Self
    {

        wpp!(self)

    }

    fn wmm(&mut self) -> Self
    {

        wmm!(self)

    }

}