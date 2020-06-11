use hdk::prelude::*;
use holochain_turn_based_game::game::Game as TurnBasedGame;
use holochain_turn_based_game::game::GameEntry;

use hdk::holochain_persistence_api::cas::content::Address;

use chess::{ChessMove, Color, Game, GameResult, Square};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};

#[derive(Clone, Debug)]
pub struct ChessGame {
    pub white_address: Address,
    pub black_address: Address,

    pub game: Game,
}

impl Into<String> for ChessGame {
    fn into(self) -> String {
        format!("{}", self.game.current_position())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub enum ChessGameMove {
    PlacePiece { from: String, to: String },
    Resign,
}

impl Into<String> for ChessGameMove {
    fn into(self) -> String {
        match self {
            ChessGameMove::PlacePiece { from, to } => format!("{}-{}", from, to),
            ChessGameMove::Resign => String::from("resign"),
        }
    }
}

impl TurnBasedGame<ChessGameMove> for ChessGame {
    fn min_players() -> Option<usize> {
        Some(2)
    }
    fn max_players() -> Option<usize> {
        Some(2)
    }
    fn initial(players: &Vec<Address>) -> Self {
        ChessGame {
            white_address: players[0].clone(),
            black_address: players[1].clone(),

            game: Game::new(),
        }
    }
    fn is_valid(self, _game_move: ChessGameMove) -> Result<(), String> {
        // TODO: Validate movement
        Ok(())
    }
    fn apply_move(
        &mut self,
        game_move: &ChessGameMove,
        _player_index: usize,
        author_address: &Address,
    ) -> () {
        match game_move {
            ChessGameMove::PlacePiece { from, to } => {
                let chess_move = ChessMove::new(
                    Square::from_string(from.clone()).unwrap(),
                    Square::from_string(to.clone()).unwrap(),
                    None,
                );

                self.game.make_move(chess_move);
            }
            ChessGameMove::Resign => {
                let resigner_color = match author_address.clone() == self.white_address {
                    true => Color::White,
                    false => Color::Black,
                };
                self.game.resign(resigner_color);
            }
        }
        ()
    }

    // Gets the winner for the game
    fn get_winner(
        &self,
        _moves_with_author: &Vec<(Address, ChessGameMove)>,
        players: &Vec<Address>,
    ) -> Option<Address> {
        match self.game.result() {
            Some(result) => match result {
                GameResult::WhiteCheckmates | GameResult::BlackResigns => Some(players[0].clone()),
                _ => Some(players[1].clone()),
            },
            None => None,
        }
    }
}

pub fn create_game(rival: Address, timestamp: u64) -> ZomeApiResult<Address> {
    let game = GameEntry {
        players: vec![rival.clone(), hdk::AGENT_ADDRESS.clone()],
        created_at: timestamp,
    };

    holochain_turn_based_game::create_game(game)
}

pub fn make_move(game_address: Address, from: String, to: String) -> ZomeApiResult<Address> {
    let game_move = ChessGameMove::PlacePiece { from, to };
    holochain_turn_based_game::create_move(&game_address, game_move)
}
