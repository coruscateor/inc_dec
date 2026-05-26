#[cfg(test)]
mod tests
{

    use crate::{IncDecSelf, mm, mmf, pp, ppf};

    //use super::*;

    #[test]
    fn try_pp()
    {

        let mut int_val = 1;

        pp!(int_val);

        assert_eq!(2, int_val);

    }

    #[test]
    fn try_ppf()
    {

        let mut f32_val: f32 = 1.0;

        ppf!(f32_val);

        assert_eq!(2.0, f32_val);

        let mut f64_val = 1.0;

        ppf!(f64_val);

        assert_eq!(2.0, f64_val);

    }

    #[test]
    fn try_mm()
    {

        let mut int_val = 2;

        mm!(int_val);

        assert_eq!(1, int_val);

    }

    #[test]
    fn try_mmf()
    {

        let mut f32_val: f32 = 2.0;

        mmf!(f32_val);

        assert_eq!(1.0, f32_val);

        let mut f64_val = 2.0;

        mmf!(f64_val);

        assert_eq!(1.0, f64_val);

    }

    #[test]
    fn pp_and_mm_methods()
    {

        let mut u32_val: u32 = 0;

        assert_eq!(1, u32_val.pp());

        assert_eq!(0, u32_val.mm());

    }

    #[test]
    fn try_pp_and_try_mm_methods()
    {

        let mut u32_val: u32 = 0;

        assert_eq!(Some(1), u32_val.try_pp());

        assert_eq!(Some(0), u32_val.try_mm());

        assert_eq!(None, u32_val.try_mm());

    }
    
}