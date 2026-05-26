
///
/// For implementing incrementation and decrementation on Self.
/// 
pub trait IncDecExt
    where Self: Sized + Copy
{

    fn pp(&mut self) -> Self;

    fn try_pp(&mut self) -> Option<Self>;

    fn mm(&mut self) -> Self;

    fn try_mm(&mut self) -> Option<Self>;

}



///
/// For implementing integer-only incrementation and decrementation on Self.
/// 
pub trait IntIncDecExt
    where Self: Sized + Copy
{

    fn opp(&mut self) -> (Self, bool);

    fn omm(&mut self) -> (Self, bool);

    fn wpp(&mut self) -> Self;

    fn wmm(&mut self) -> Self;

}
