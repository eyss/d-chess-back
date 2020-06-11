#![feature(proc_macro_hygiene)]

extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_core_types;
use hdk::prelude::*;
use hdk::AGENT_ADDRESS;
use hdk_proc_macros::zome;

extern crate serde;
// #[macro_use]
extern crate serde_derive;
extern crate serde_json;
// #[macro_use]
extern crate chess;
extern crate holochain_json_derive;
extern crate holochain_turn_based_game;
extern crate rand;

mod chess_game;
mod invitation;

use chess_game::*;
use invitation::Invitation;
// use holochain_turn_based_game::game::GameEntry;

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
use hdk::holochain_persistence_api::cas::content::Address;

// see https://developer.holochain.org/api/0.0.47-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

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
        holochain_turn_based_game::game_definition::<ChessGame, ChessGameMove>()
    }
    #[entry_def]
    fn move_def() -> ValidatingEntryType {
        holochain_turn_based_game::move_definition::<ChessGame, ChessGameMove>()
    }

    #[entry_def]
    fn invitation_def() -> ValidatingEntryType {
        invitation::entry_def()
    }

    #[zome_fn("hc_public")]
    fn get_my_public_address() -> ZomeApiResult<Address> {
        Ok(AGENT_ADDRESS.clone())
    }

    #[zome_fn("hc_public")]
    fn invite_user(opponent: Address, timestamp: u64) -> ZomeApiResult<bool> {
        let invitation = Invitation {
            invited: opponent.clone(),
            inviter: AGENT_ADDRESS.clone(),
            status: String::from("Pending"),
            timestamp,
        };
        let entry = invitation.entry();
        let invitation_address = hdk::commit_entry(&entry)?;
        // TODO: link
        hdk::link_entries(&AGENT_ADDRESS, &invitation_address, "inviter", "")?;
        hdk::link_entries(&opponent, &invitation_address, "invited", "")?;
        Ok(true)
    }
    #[zome_fn("hc_public")]
    fn get_sent_invitations() -> ZomeApiResult<Vec<Invitation>> {
        let res = hdk::utils::get_links_and_load_type(
            &AGENT_ADDRESS,
            LinkMatch::Exactly("inviter"),
            LinkMatch::Any,
        )?;
        Ok(res)
    }
    #[zome_fn("hc_public")]
    fn get_received_invitations() -> ZomeApiResult<Vec<Invitation>> {
        let res = hdk::utils::get_links_and_load_type(
            &AGENT_ADDRESS,
            LinkMatch::Exactly("invited"),
            LinkMatch::Any,
        )?;
        Ok(res)
    }
    #[zome_fn("hc_public")]
    fn reject_invitation(
        inviter: Address,
        invited: Address,
        invitation_timestamp: u64,
    ) -> ZomeApiResult<bool> {
        let invitation = Invitation {
            inviter,
            invited,
            status: "Pending".to_string(),
            timestamp: invitation_timestamp,
        };
        let invitation_entry = invitation.entry();
        let invitation_address = hdk::entry_address(&invitation_entry)?;
        let mut invitation = hdk::utils::get_as_type::<Invitation>(invitation_address.clone())?;
        invitation.status = String::from("rejected");
        let entry = invitation.entry();
        let _ = hdk::api::update_entry(entry, &invitation_address);
        Ok(true)
    }
    #[zome_fn("hc_public")]
    fn accept_invitation(
        inviter: Address,
        invited: Address,
        invitation_timestamp: u64,
        timestamp: u64,
    ) -> ZomeApiResult<bool> {
        let invitation = Invitation {
            inviter,
            invited,
            status: "Pending".to_string(),
            timestamp: invitation_timestamp,
        };
        let invitation_entry = invitation.entry();
        let invitation_address = hdk::entry_address(&invitation_entry)?;
        let mut invitation = hdk::utils::get_as_type::<Invitation>(invitation_address.clone())?;
        invitation.status = String::from("accepted");
        let entry = invitation.clone().entry();
        let _ = hdk::api::update_entry(entry, &invitation_address);
        let _ = create_game(invitation.inviter, timestamp)?;
        Ok(true)
    }
    // This must be coupled asynchronically with the get_entry function
    // to load the game data in an asynchronic manner.
    // In order to get the rival username, use the get_username function
    #[zome_fn("hc_public")]
    fn get_my_games() -> ZomeApiResult<Vec<Address>> {
        holochain_turn_based_game::get_agent_games(&AGENT_ADDRESS)
    }

    #[zome_fn("hc_public")]
    fn get_entry(entry_address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&entry_address)
    }

    #[zome_fn("hc_public")]
    fn get_game_state(game_address: Address) -> ZomeApiResult<String> {
        let chess_game_state: ChessGame = holochain_turn_based_game::get_game_state(&game_address)?;
        Ok(chess_game_state.into())
    }

    #[zome_fn("hc_public")]
    fn get_game_moves(game_address: Address) -> ZomeApiResult<Vec<String>> {
        let moves: Vec<ChessGameMove> = holochain_turn_based_game::get_game_moves(&game_address)?;

        Ok(moves.into_iter().map(|m| m.into()).collect())
    }

    #[zome_fn("hc_public")]
    fn make_move(game_address: Address, from: String, to: String) -> ZomeApiResult<Address> {
        chess_game::make_move(game_address, from, to)
    }

    #[zome_fn("hc_public")]
    fn surrender(game_address: Address) -> ZomeApiResult<Address> {
        let game_move = ChessGameMove::Resign;
        holochain_turn_based_game::create_move(&game_address, game_move)
    }
}
