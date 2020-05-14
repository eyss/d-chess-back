extern crate hdk;
extern crate hdk_proc_macros;

extern crate holochain_json_derive;
extern crate holochain_turn_based_game;

use hdk::prelude::*;
use hdk::prelude::DefaultJson;

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct User {
    pub username: String,
}

impl User{
    pub fn entry(self) -> Entry {
        Entry::App("user".into(), self.into())
    }
}

impl From<String> for User {
    fn from(username: String) -> Self {
        User { username }
    }
}

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: "user",
        description: "user profile",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<User>| {
            Ok(())
        },
        links: [
            from!(
                "%agent_id",
                link_type: "agent->user",
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