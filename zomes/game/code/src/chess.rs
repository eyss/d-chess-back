extern crate hdk;
extern crate hdk_proc_macros;

extern crate holochain_json_derive;
extern crate holochain_turn_based_game;

use holochain_turn_based_game::game::Game;

use hdk::holochain_persistence_api::{
    cas::content::Address,
};


use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};


#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct ChessGame {
    pub white: Vec<ChessMove>,
    pub black: Vec<ChessMove>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Move {
    pub origin_number: u8,
    pub origin_letter: u8,
    pub destination_number: u8,
    pub destination_letter: u8,
}
#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub enum ChessMove {
  Place(Move),
  Resign,
}

impl Game<ChessMove> for ChessGame {
    fn min_players() -> Option<usize> {
        Some(2)
    }
    fn max_players() -> Option<usize> {
        Some(2)
    }
    fn initial(_players: &Vec<Address>) -> Self {
        ChessGame {
            white: vec![],
            black: vec![],
        }
    }
    fn is_valid(self, _game_move: ChessMove) -> Result<(), String> {
        // TODO: Validate movement
        Ok(())
    }
    fn apply_move(
      &mut self,
      game_move: &ChessMove,
      player_index: usize,
      _author_address: &Address,
    ) -> () {
        match player_index {
            0 => {
                self.white.push(game_move.clone())
            },
            1 => {
                self.black.push(game_move.clone())
            },
            _ => {}
        }
        ()
    }

    // Gets the winner for the game
    fn get_winner(
      &self,
      _moves_with_author: &Vec<(Address, ChessMove)>,
      _players: &Vec<Address>,
    ) -> Option<Address> {
        // TODO: get winner
        None
    }
}