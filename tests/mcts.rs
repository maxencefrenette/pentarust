use float_ord::FloatOrd;
use pentarust::game::Board;
use pentarust::game::Player;
use pentarust::mcts::TreeNode;
use std::time::Duration;

#[test]
#[ignore]
fn mcts_test() {
    let mut tree = TreeNode::new(Board::new([
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
    ]));
    tree.search(Duration::from_secs(2));

    println!("Total Games: {}", tree.win_stats.games_played);
    println!(
        "Player1 win percentage: {:.2}%",
        tree.win_stats.expected_win_ratio(Player::Player1) * 100.
    );
    println!(
        "Player2 win percentage: {:.2}%",
        tree.win_stats.expected_win_ratio(Player::Player2) * 100.
    );
    println!();

    println!("Main Line:");

    let mut cur = &tree;
    while cur.children.as_ref().filter(|c| !c.is_empty()).is_some() {
        println!("{:?}", cur.best_move());
        let player = cur.state.turn();
        cur = cur
            .children
            .as_ref()
            .expect("non-empty tree")
            .iter()
            .max_by_key(|child| FloatOrd(child.win_stats.expected_win_ratio(player)))
            .expect("non-empty list");
    }
    println!("Outcome: {:?}", cur.state.outcome());
    println!("Final State: {:?}", cur.state);
}
