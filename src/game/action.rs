use crate::game::Swap;

pub struct Action {
    square: u8,
    swap: Swap,
}

impl Action {
    pub fn new(square: u8, swap: Swap) -> Action {
        Action { square, swap }
    }
}
