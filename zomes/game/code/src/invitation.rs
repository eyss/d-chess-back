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
pub struct Invitation {
    pub inviter: Address,
    pub invited: Address,
    pub status: String
}

impl Invitation{
    pub fn entry(self) -> Entry {
        Entry::App("invitation".into(), self.into())
    }
}

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: "invitation",
        description: "An invitation from one user to another",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Invitation>| {
            Ok(())
        },
        links: [
            from!(
                "%agent_id",
                link_type: "invited",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "%agent_id",
                link_type: "inviter",
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