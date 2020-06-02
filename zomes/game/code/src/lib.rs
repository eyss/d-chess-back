#![feature(proc_macro_hygiene)]

extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_core_types;
use hdk::prelude::*;
use hdk_proc_macros::zome;
use hdk::AGENT_ADDRESS;

extern crate serde;
// #[macro_use]
extern crate serde_derive;
extern crate serde_json;
// #[macro_use]
extern crate holochain_json_derive;
extern crate holochain_turn_based_game;
extern crate rand;

mod invitation;
mod chess;
mod user;

use invitation::Invitation;
use chess::*;
use user::User;
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
use hdk::holochain_persistence_api::{
    cas::content::Address,
};

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
        holochain_turn_based_game::game_definition::<ChessGame, ChessMove>()
    }
    #[entry_def]
    fn move_def() -> ValidatingEntryType {
        holochain_turn_based_game::move_definition::<ChessGame, ChessMove>()
    }
    #[entry_def]
    fn user_def() -> ValidatingEntryType {
        user::entry_def()
    }
    #[entry_def]
    fn invitation_def() -> ValidatingEntryType {
        invitation::entry_def()
    }
    #[entry_def]
    fn match_data_def() -> ValidatingEntryType {
        chess::match_data_def()
    }
    #[zome_fn("hc_public")]
    fn get_my_public_address() -> ZomeApiResult<Address> {
        Ok(AGENT_ADDRESS.clone())
    }
    #[zome_fn("hc_public")]
    fn create_profile(username: String) -> ZomeApiResult<Address> {
        let user = User::from(username);
        let profile_entry = user.entry();
        let profile_address = hdk::commit_entry(&profile_entry)?;
        hdk::link_entries(&AGENT_ADDRESS, &profile_address, "agent->user", "")?;
        Ok(AGENT_ADDRESS.clone())
    }
    #[zome_fn("hc_public")]
    fn get_username(addr: Address) -> ZomeApiResult<String> {
        let wrapped_profile_array: Vec<User> = hdk::utils::get_links_and_load_type(
            &addr,
            LinkMatch::Exactly("agent->user"),
            LinkMatch::Any,
        )?;
        let wrapped_profile = wrapped_profile_array.last();
        match wrapped_profile {
            Some(profile) => Ok(profile.username.clone()),
            None => Err(ZomeApiError::Internal("profile not found".to_string())),
        }
    }
    #[zome_fn("hc_public")]
    fn get_my_profile() -> ZomeApiResult<User> {
        //fetch profile linked from the agent address
        let mut res = hdk::utils::get_links_and_load_type::<User>(
            &AGENT_ADDRESS,
            LinkMatch::Exactly("agent->user"),
            LinkMatch::Any,
        )?;

        match res.pop() {
            Some(profile) => Ok(User {
                username: profile.username,
            }),
            None => Err(ZomeApiError::Internal("No profile registered".to_string())),
        }
    }
    #[zome_fn("hc_public")]
    fn invite_user(username: String, timestamp: u64) -> ZomeApiResult<bool> {
        let user = User::from(username);
        let user_entry = user.entry();
        let user_entry_address = hdk::entry_address(&user_entry)?;
        let status_request = StatusRequestKind::Latest;
        let options = GetEntryOptions::new(
            status_request,
            false,
            true,
            holochain_core_types::time::Timeout::new(std::usize::MAX)
        );
        let metadata: GetEntryResultType = (hdk::api::get_entry_result(&user_entry_address,options)?).result;

        let rival: Address;
        match metadata{
            GetEntryResultType::Single(data) => {
                if data.headers.len() == 0 {
                    return Err(ZomeApiError::Internal("User not found".to_string()))
                }
                let provenance = &data.headers[0].provenances()[0];
                rival = provenance.0.clone();
            },
            GetEntryResultType::All(_) => {
                return Err(ZomeApiError::Internal("How did this even happen".to_string()))
            }
        }
        let invitation = Invitation {
            invited: rival.clone(),
            inviter: AGENT_ADDRESS.clone(),
            status: String::from("Pending"),
            timestamp,
        };
        let entry = invitation.entry();
        let invitation_address = hdk::commit_entry(&entry)?;
        // TODO: link
        hdk::link_entries(&AGENT_ADDRESS, &invitation_address, "inviter", "")?;
        hdk::link_entries(&rival, &invitation_address, "invited", "")?;
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
    fn reject_invitation(inviter: Address, invited: Address, invitation_timestamp: u64) -> ZomeApiResult<bool> {
        let invitation = Invitation{
            inviter,
            invited,
            status: "Pending".to_string(),
            timestamp: invitation_timestamp
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
    fn accept_invitation(inviter: Address, invited: Address, invitation_timestamp: u64, timestamp: u64) -> ZomeApiResult<bool> {
        let invitation = Invitation{
            inviter,
            invited,
            status: "Pending".to_string(),
            timestamp: invitation_timestamp
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
    fn check_my_games() -> ZomeApiResult<Vec<MatchData>> {
        let res = hdk::utils::get_links_and_load_type(
            &AGENT_ADDRESS,
            LinkMatch::Exactly("agent->match_data"),
            LinkMatch::Any,
        )?;
        Ok(res)
    }
    #[zome_fn("hc_public")]
    fn get_entry(game_address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&game_address)
    }
    #[zome_fn("hc_public")]
    fn place_piece(game_address: Address, origin_number: u8, origin_letter: u8, destination_number: u8, destination_letter: u8) -> ZomeApiResult<Address> {
        let new_move = Move {
            origin_number,
            origin_letter,
            destination_number,
            destination_letter,
        };
        let game_move = ChessMove::Place(new_move);
        holochain_turn_based_game::create_move(&game_address, game_move)
    }
    #[zome_fn("hc_public")]
    fn surrender(game_address: Address) -> ZomeApiResult<Address> {
        let game_move = ChessMove::Resign;
        holochain_turn_based_game::create_move(&game_address, game_move)
    }
}