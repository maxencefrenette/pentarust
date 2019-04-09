use pentarust::game::Board;
use pentarust::game::Player;
use rusqlite::types::ToSql;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Node {
    pub board: Board,
    pub games_played: i64,
    pub player1_wins: i64,
    pub player2_wins: i64,
    pub expanded: bool,
}

impl Node {
    pub fn insert(conn: &Connection, node: &Node) -> rusqlite::Result<()> {
        conn.execute(
            "INSERT INTO node
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            &[
                &(node.board.player1 as i64) as &ToSql,
                &(node.board.player2 as i64),
                &(node.board.player1 | node.board.player2).count_ones(),
                &node.games_played,
                &node.player1_wins,
                &node.player2_wins,
                &node.expanded,
            ],
        )?;

        Ok(())
    }

    pub fn get(conn: &Connection, board: Board) -> rusqlite::Result<Node> {
        conn.query_row(
            "SELECT *
            FROM node
            WHERE player1=?1 AND PLAYER2=?2",
            &[board.player1 as i64, board.player2 as i64],
            |row| {
                Ok(Node {
                    board: Board {
                        player1: row.get::<usize, i64>(0)? as u64,
                        player2: row.get::<usize, i64>(1)? as u64,
                    },
                    games_played: row.get(3)?,
                    player1_wins: row.get(4)?,
                    player2_wins: row.get(5)?,
                    expanded: row.get(6)?,
                })
            },
        )
    }

    pub fn update(&self, conn: &Connection) -> rusqlite::Result<()> {
        conn.execute(
            "UPDATE node
            SET games_played=?3, player1_wins=?4, player2_wins=?5, expanded=?6
            WHERE player1=?1 AND PLAYER2=?2",
            &[
                &(self.board.player1 as i64) as &ToSql,
                &(self.board.player2 as i64),
                &self.games_played,
                &self.player1_wins,
                &self.player2_wins,
                &self.expanded,
            ],
        )?;

        Ok(())
    }

    pub fn ucb(&self, total_games: i64) -> f64 {
        let c = f64::sqrt(2.);
        let w = match self.board.turn() {
            Player::Player1 => self.player1_wins,
            Player::Player2 => self.player2_wins,
        } as f64;
        let n = self.games_played as f64;

        (w / n) + c * f64::sqrt((total_games as f64).ln() / n)
    }
}
