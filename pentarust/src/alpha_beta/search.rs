use crate::alpha_beta::eval;
use crate::alpha_beta::TranspositionTable;
use crate::game::Action;
use crate::game::Board;
use crate::game::Outcome;
use crate::game::Player;
use crate::game::Swap;
use rand::thread_rng;
use rand::Rng;
use std::time::Duration;
use std::time::SystemTime;

pub fn search(board: Board, duration: Duration, transpo_table: &mut TranspositionTable) -> Action {
    if board == Board::default() {
        return Action::new(7, Swap::BL_BR);
    }

    let start = SystemTime::now();
    let mut depth = 3;
    let mut children: Vec<(Board, i32)> =
        board.children(false).into_iter().map(|c| (c, 0)).collect();

    loop {
        let best_action = board.action_to(children[0].0);
        let best_eval = children[0].1;

        let mut alpha = -1_000_000;
        let beta = 999_950; // Any guaranteed win is good

        for c in children.iter_mut() {
            let early_stop = || start.elapsed().unwrap_or(Duration::from_secs(0)) > duration;

            let search_result = negamax(c.0, depth, alpha, beta, transpo_table, &early_stop);
            let score = if let Some(value) = search_result {
                -value
            } else {
                println!("Searched depth {}", depth);
                println!("Eval {}", best_eval);
                return best_action;
            };

            if score >= beta {
                println!("Found a guaranteed win at depth {} !", depth + 1);
                return board.action_to(c.0);
            }

            if score > alpha {
                alpha = score
            }

            c.1 = score;
        }

        let mut rng = thread_rng();
        children.sort_by_key(|(_c, eval)| -eval + rng.gen_range(-10, 10));

        depth += 1;
    }
}

/// Returns some(value) if the calculation has time to finish, None otherwise
pub fn negamax<F>(
    board: Board,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    transpo_table: &mut TranspositionTable,
    early_stop: &F,
) -> Option<i32>
where
    F: Fn() -> bool,
{
    if depth >= 3 && early_stop() {
        return None;
    }

    if let Some(outcome) = board.outcome() {
        let m = if board.turn() == Player::Player1 {
            1
        } else {
            -1
        };

        let win_score = 1_000_000 - board.turn_number() as i32;
        let value = match outcome {
            Outcome::Player1Win => m * win_score,
            Outcome::Player2Win => -m * win_score,
            Outcome::Draw => 0,
        };
        return Some(value);
    }

    if depth == 0 {
        return Some(eval(board));
    }

    let mut max_score: i32 = 1_000_000 - board.turn_number() as i32 - 1;
    if let Some(upper_bound) = transpo_table.get(board, depth) {
        max_score = upper_bound;
    }

    if beta > max_score {
        beta = max_score;
        if alpha >= beta {
            return Some(beta);
        }
    }

    let mut children = board.children(false);

    // Order moves based on previous iterations
    if depth > 3 {
        children.sort_by_key(|c| transpo_table.get(*c, depth - 1).unwrap_or(0));
    }

    for c in children.into_iter() {
        let search_result = negamax(c, depth - 1, -beta, -alpha, transpo_table, early_stop);
        let score = if let Some(value) = search_result {
            -value
        } else {
            return None;
        };

        if score >= beta {
            return Some(score);
        }

        if score > alpha {
            alpha = score
        }
    }

    transpo_table.put(board, alpha, depth);
    Some(alpha)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn negamax_test1() {
        let board = Board::new([
            [1, 2, 0, 0, 0, 0],
            [1, 2, 0, 1, 2, 0],
            [1, 2, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
        ]);

        assert_eq!(
            negamax(
                board,
                3,
                -1_000_000,
                1_000_000,
                &mut TranspositionTable::new(1),
                &|| false
            )
            .unwrap(),
            999_991
        );
    }

    #[test]
    fn negamax_test2() {
        let board = Board::new([
            [1, 0, 0, 1, 2, 1],
            [0, 1, 0, 0, 2, 0],
            [0, 0, 0, 0, 2, 0],
            [0, 0, 0, 1, 1, 0],
            [0, 2, 0, 0, 2, 0],
            [0, 0, 0, 0, 0, 0],
        ]);

        assert_eq!(
            negamax(
                board,
                3,
                -1_000_000,
                1_000_000,
                &mut TranspositionTable::new(1),
                &|| false
            )
            .unwrap(),
            999_988
        );
    }
}
