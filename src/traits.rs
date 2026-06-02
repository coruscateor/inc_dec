
///
/// For implementing incrementation and decrementation on numeric types.
/// 
pub trait IncDecExt
    where Self: Sized + Copy
{

    ///
    /// Increment the value by one, returning a copy of Self.
    /// 
    fn pp(&mut self) -> Self;

    ///
    /// Try to increment the value by one, returning a copy of Self if successful.
    /// 
    fn try_pp(&mut self) -> Option<Self>;

    ///
    /// Decrement the value by one, returning a copy of Self.
    /// 
    fn mm(&mut self) -> Self;

    ///
    /// Try to decrement the value by one, returning a copy of Self if successful.
    /// 
    fn try_mm(&mut self) -> Option<Self>;

}



///
/// For implementing incrementation and decrementation on integer types.
/// 
pub trait IntIncDecExt
    where Self: Sized + Copy
{

    ///
    /// Overflowing-increment the value by one, returning a copy of Self and a boolean indicating whether or not an overflow would’ve occurred in a tuple. If an overflow would’ve occurred then a wrapped copy of Self is returned.
    /// 
    fn opp(&mut self) -> (Self, bool);

    ///
    /// Overflowing-decrement the value by one, returning a copy of Self and a boolean indicating whether or not an overflow would’ve occurred in a tuple. If an overflow would’ve occurred then a wrapped copy of Self is returned.
    /// 
    fn omm(&mut self) -> (Self, bool);

    ///
    /// Wrapping-increment the value by one, returning a copy of Self.
    /// 
    fn wpp(&mut self) -> Self;

    ///
    /// Wrapping-decrement the value by one, returning a copy of Self.
    /// 
    fn wmm(&mut self) -> Self;

}
