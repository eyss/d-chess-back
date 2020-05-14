extern crate hdk;
extern crate hdk_proc_macros;

extern crate holochain_json_derive;
extern crate holochain_turn_based_game;

use hdk::AGENT_ADDRESS;

use hdk::prelude::*;
use hdk::prelude::DefaultJson;
use holochain_turn_based_game::game::Game;
use holochain_turn_based_game::game::GameEntry;

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
pub struct MatchData {
    pub match_address: Address,
}

impl MatchData{
    pub fn entry(self) -> Entry {
        Entry::App("match_data".into(), self.into())
    }
}
pub fn match_data_def () -> ValidatingEntryType {
    entry!(
        name: "match_data",
        description: "data structure for interacting with the game through links",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<MatchData>| {
            Ok(())
        },
        links: [
            from!(
                "%agent_id",
                link_type: "agent->match_data",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
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

pub fn create_game( rival: Address, timestamp: u32) -> ZomeApiResult<()>{
    let game = GameEntry {
        players: vec![rival.clone(), hdk::AGENT_ADDRESS.clone()],
        created_at: timestamp,
    };

    let match_address = holochain_turn_based_game::create_game(game);
    let match_data = MatchData {
        match_address: match_address.unwrap(),
    };

    let entry = match_data.entry();
    let match_data_address = hdk::commit_entry(&entry)?;

    hdk::link_entries(&AGENT_ADDRESS, &match_data_address, "agent->match_data", "")?;
    hdk::link_entries(&rival, &match_data_address, "agent->match_data", "")?;
    Ok(())
}