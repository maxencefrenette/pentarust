use self::Swap::*;
use std::slice::Iter;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Swap {
    TL_TR,
    BL_BR,
    TL_BL,
    TR_BR,
    TL_BR,
    TR_BL,
}

impl Swap {
    pub fn iterator() -> Iter<'static, Swap> {
        static SWAPS: [Swap; 6] = [TL_TR, BL_BR, TL_BL, TR_BR, TL_BR, TR_BL];
        SWAPS.iter()
    }
}
