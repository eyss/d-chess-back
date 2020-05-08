#![feature(proc_macro_hygiene)]

extern crate hdk;
extern crate hdk_proc_macros;
use hdk::prelude::{
    ValidatingEntryType,
    ZomeApiResult,
};
use hdk_proc_macros::zome;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;
extern crate holochain_turn_based_game;
extern crate rand;

// use rand::Rng;
/*
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};
*/
use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use holochain_turn_based_game::game::Game;

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};


// see https://developer.holochain.org/api/0.0.47-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry
#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Piece {
    pub number: u8,
    pub letter: u8,
    pub kind: String,
}

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

#[zome]
mod scores {
    #[init]
    fn init() {
        Ok(())
    }
    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }
    #[entry_def]
    fn game_def() -> ValidatingEntryType {
        holochain_turn_based_game::game_definition::<ChessGame, ChessMove>()
    }    
    #[entry_def]
    fn move_def() -> ValidatingEntryType {
        holochain_turn_based_game::move_definition::<ChessGame, ChessMove>()
    }
    #[zome_fn("hc_public")]
    fn invite_user(_addr: Address) -> ZomeApiResult<bool> {
        // TODO: invite user
        Ok(true)
    }
    // TODO: check my games
    // TODO: check my received invitations
    // TODO: accept invitation
    // TODO: check my sent unapproved invitations
    // TODO: move piece
    #[zome_fn("hc_public")]
    fn move_piece(_game_addr: Address, _movement: ChessMove) -> ZomeApiResult<bool> {
        // TODO: make a move
        Ok(true)
    }
}