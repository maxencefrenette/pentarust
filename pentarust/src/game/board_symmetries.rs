use crate::game::Board;

macro_rules! map_board {
    ($a:ident, $b:ident) => {
        pub fn $a(board: Board) -> Board {
            Board {
                player1: $b(board.player1),
                player2: $b(board.player2),
            }
        }
    };
}

map_board!(flip_vertical, flip_vertical_u64);
map_board!(flip_horizontal, flip_horizontal_u64);
map_board!(flip_diagonal, flip_diagonal_u64);
map_board!(flip_antidiagonal, flip_antidiagonal_u64);

fn flip_vertical_u64(mut x: u64) -> u64 {
    const K1: u64 = 0b000000_000000_111111_000000_000000_111111;
    const NOT_K1: u64 = !(K1 | (K1 << 12));
    const K2: u64 = 0b000000_000000_000000_111111_111111_111111;
    x = ((x >> 12) & K1) | (x & NOT_K1) | ((x & K1) << 12);
    x = (x >> 18) | ((x & K2) << 18);
    x
}

fn flip_horizontal_u64(mut x: u64) -> u64 {
    const K1: u64 = 0b001001_001001_001001_001001_001001_001001;
    const NOT_K1: u64 = !(K1 | (K1 << 2));
    const K2: u64 = 0b000111_000111_000111_000111_000111_000111;
    x = ((x >> 2) & K1) | (x & NOT_K1) | ((x & K1) << 2);
    x = (x >> 3) | ((x & K2) << 3);
    x
}

fn flip_diagonal_u64(mut x: u64) -> u64 {
    const K1: u64 = 0b000000_001001_010010_000000_001001_010010;
    const NOT_K1: u64 = !(K1 | (K1 << 7));
    const K2: u64 = 0b000000_000000_001001_000000_000000_001001;
    const NOT_K2: u64 = !(K2 | (K2 << 14));
    const K3: u64 = 0b000000_000000_000000_000111_000111_000111;
    const NOT_K3: u64 = !(K3 | (K3 << 21));
    x = ((x >> 7) & K1) | (x & NOT_K1) | ((x & K1) << 7);
    x = ((x >> 14) & K2) | (x & NOT_K2) | ((x & K2) << 14);
    x = (x >> 21) | (x & NOT_K3) | ((x & K3) << 21);
    x
}

fn flip_antidiagonal_u64(mut x: u64) -> u64 {
    const K1: u64 = 0b000000_100100_010010_000000_100100_010010;
    const NOT_K1: u64 = !(K1 | (K1 << 5));
    const K2: u64 = 0b000000_000000_100100_000000_000000_100100;
    const NOT_K2: u64 = !(K2 | (K2 << 10));
    const K3: u64 = 0b000000_000000_000000_111000_111000_111000;
    const NOT_K3: u64 = !(K3 | (K3 << 15));
    x = ((x >> 5) & K1) | (x & NOT_K1) | ((x & K1) << 5);
    x = ((x >> 10) & K2) | (x & NOT_K2) | ((x & K2) << 10);
    x = (x >> 15) | (x & NOT_K3) | ((x & K3) << 15);
    x
}

// fn rotate_clockwise_u64(x: u64) -> u64 {
//     flip_vertical_u64(flip_antidiagonal_u64(x))
// }

// fn rotate180_u64(x: u64) -> u64 {
//     flip_vertical_u64(flip_horizontal_u64(x))
// }

// fn rotate_anticlockwise_u64(x: u64) -> u64 {
//     flip_vertical_u64(flip_diagonal_u64(x))
// }

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref BOARD1: Board = Board::new([
            [1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0]
        ]);
    }

    #[test]
    fn vertical_flip_test() {
        assert_eq!(
            flip_vertical(*BOARD1),
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0],
                [0, 1, 1, 0, 0, 0],
                [1, 1, 1, 0, 0, 0]
            ])
        );
    }

    #[test]
    fn horizontal_flip_test() {
        assert_eq!(
            flip_horizontal(*BOARD1),
            Board::new([
                [0, 0, 0, 1, 1, 1],
                [0, 0, 0, 1, 1, 0],
                [0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
        );
    }

    #[test]
    fn flip_diagonal_test() {
        assert_eq!(
            flip_diagonal(*BOARD1),
            Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 1],
                [0, 0, 0, 0, 1, 1],
                [0, 0, 0, 0, 0, 1]
            ])
        );

        assert_eq!(
            flip_diagonal(Board::new([
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0]
            ])),
            Board::new([
                [0, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
        );
    }

    #[test]
    fn flip_antidiagonal_test() {
        assert_eq!(
            flip_antidiagonal(*BOARD1),
            Board::new([
                [1, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 0, 0],
                [1, 1, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0]
            ])
        );
    }
}
