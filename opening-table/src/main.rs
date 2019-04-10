use float_ord::FloatOrd;
use node::Node;
use pentarust::best_move;
use pentarust::game::Board;
use pentarust::game::Outcome;
use pentarust::game::Player;
use rand::thread_rng;
use rand::Rng;
use rusqlite::OptionalExtension;
use rusqlite::{Connection, NO_PARAMS};
use std::io::stdout;
use std::io::Write;
use std::time::Duration;
use structopt::StructOpt;

mod node;

const GAMES_PER_NEW_NODE: i64 = 1;

#[derive(StructOpt)]
#[structopt(name = "opening-table")]
enum Options {
    #[structopt(name = "init")]
    Init,
    #[structopt(name = "stats")]
    Stats,
    #[structopt(name = "generate")]
    Generate,
    #[structopt(name = "main-line")]
    MainLine,
}

fn main() -> rusqlite::Result<()> {
    let opt = Options::from_args();
    let conn = Connection::open("opening-table.sqlite")?;

    match opt {
        Options::Init => init(&conn)?,
        Options::Stats => stats(&conn)?,
        Options::Generate => generate(&conn)?,
        Options::MainLine => main_line(&conn)?,
    };

    Ok(())
}

fn init(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DROP TABLE IF EXISTS node", NO_PARAMS)?;
    conn.execute(
        "CREATE TABLE node (
            player1 INTEGER NOT NULL,
            player2 INTEGER NOT NULL,
            turn INTEGER NOT NULL,
            games_played INTEGER NOT NULL,
            player1_wins INTEGER NOT NULL,
            player2_wins INTEGER NOT NULL,
            expanded INTEGER NOT NULL,
            PRIMARY KEY (player1, player2)
        )",
        NO_PARAMS,
    )?;
    conn.execute(
        "INSERT INTO node
        VALUES (0, 0, 0, 0, 0, 0, 0)",
        NO_PARAMS,
    )?;

    Ok(())
}

fn stats(conn: &Connection) -> rusqlite::Result<()> {
    println!("{:#?}", Node::get(conn, Board::default())?);
    Ok(())
}

/// Repeatedly expands the tree
fn generate(conn: &Connection) -> rusqlite::Result<()> {
    loop {
        expand(conn, Board::default())?;
    }
}

/// Expands the tree once
fn expand(conn: &Connection, state: Board) -> rusqlite::Result<Node> {
    let mut node = Node::get(conn, state)?;
    let mut children: Vec<Board> = state
        .children(false)
        .into_iter()
        .map(|c| c.canonical())
        .collect();

    children.sort_unstable();
    children.dedup();

    for c in children.iter() {
        if !c.is_valid() {
            panic!("Invalid children generated\n{:?}\n{:?}", state, c);
        }
    }

    let child_nodes = if node.expanded {
        let mut rng = thread_rng();
        let mut child_nodes = children
            .into_iter()
            .map(|child| Node::get(conn, child))
            .collect::<rusqlite::Result<Vec<Node>>>()?;

        let best_child_index = child_nodes
            .iter()
            .enumerate()
            .max_by_key(|(_, n)| FloatOrd(n.ucb(node.games_played) + 0.0001f64 * rng.gen::<f64>()))
            .expect("Tried to expand a terminal state")
            .0;

        child_nodes[best_child_index] = expand(conn, child_nodes[best_child_index].board)?;

        child_nodes
    } else {
        println!("Expanding {:?}", state);

        node.expanded = true;
        let child_nodes = children
            .into_iter()
            .map(|child| {
                print!(".");
                stdout().flush().expect("Failed flushing the buffer");

                if let Some(node) = Node::get(conn, child).optional()? {
                    Ok(node)
                } else {
                    let mut node = Node {
                        board: child,
                        games_played: GAMES_PER_NEW_NODE,
                        player1_wins: 0,
                        player2_wins: 0,
                        expanded: false,
                    };

                    for _ in 0..GAMES_PER_NEW_NODE {
                        match play_game(child) {
                            Outcome::Player1Win => node.player1_wins += 1,
                            Outcome::Player2Win => node.player2_wins += 1,
                            Outcome::Draw => (),
                        }
                    }

                    // Insert node in db
                    Node::insert(conn, &node)?;

                    Ok(node)
                }
            })
            .collect::<rusqlite::Result<Vec<Node>>>()?;

        println!();

        child_nodes
    };

    node.games_played = 0;
    node.player1_wins = 0;
    node.player2_wins = 0;
    for n in child_nodes.iter() {
        node.games_played += n.games_played;
        node.player1_wins += n.player1_wins;
        node.player2_wins += n.player2_wins;
    }

    node.update(conn)?;
    Ok(node)
}

fn play_game(mut board: Board) -> Outcome {
    loop {
        if let Some(outcome) = board.outcome() {
            return outcome;
        }

        board = board.apply_action(best_move(board, Duration::from_millis(100)));
    }
}

fn main_line(conn: &Connection) -> rusqlite::Result<()> {
    let mut node = Node::get(conn, Board::default())?;

    loop {
        println!(
            "{}",
            (node.player1_wins as f64) / (node.games_played as f64)
        );
        println!("{:?}", node);
        println!();

        if !node.expanded {
            break;
        }

        let mut children: Vec<Board> = node
            .board
            .children(false)
            .into_iter()
            .map(|c| c.canonical())
            .collect();

        children.sort_unstable();
        children.dedup();

        let child_nodes = children
            .into_iter()
            .map(|child| Node::get(conn, child))
            .collect::<rusqlite::Result<Vec<Node>>>()?;

        node = child_nodes
            .into_iter()
            .max_by_key(|child_node| {
                let w = match node.board.turn() {
                    Player::Player1 => child_node.player1_wins,
                    Player::Player2 => child_node.player2_wins,
                } as f64;
                let n = child_node.games_played as f64;

                FloatOrd(w / n)
            })
            .expect("Tried to expand a non-expanded state");
    }

    Ok(())
}
