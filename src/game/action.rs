use crate::game::Swap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Action {
    pub square: u8,
    pub swap: Swap,
}

impl Action {
    pub fn new(square: u8, swap: Swap) -> Action {
        Action { square, swap }
    }
}
