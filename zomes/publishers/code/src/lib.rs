#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address,
        hash::HashString,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        link::LinkMatch,
    }
};


define_zome! {
    entries: [
        entry!(
            name: "publishersGroup",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<PublisherGroup>| {
                Ok(())
            },
            links: [
                to!(
                    "publishersItem",
                    link_type: "items",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "publishersItem",
            description: "",
            sharing: Sharing::Public,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |validation_data: hdk::EntryValidationData<Publisher>| {
                Ok(())
            }
        )
    ]

    init: || {
        Ok(())
    }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
        create_group: {
            inputs: |group: PublisherGroup|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_group
        }
        get_groups: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: handle_get_groups
        }
        add_publisher: {
            inputs: |publisher_item: Publisher, publisher_addr: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add
        }
        get_publishers: {
            inputs: |publisher_addr: HashString|,
            outputs: |result: ZomeApiResult<GetPublishersResponse>|,
            handler: handle_get_all
        }
    ]
    traits: {
        hc_public [create_group, get_groups, add_publisher, get_publishers]
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct PublisherGroup {
    name: String
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct Publisher {
    name: String
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct GetPublishersResponse {
    name: String,
    items: Vec<Publisher>
}

fn handle_create_group(group: PublisherGroup) -> ZomeApiResult<Address> {
    // define the entry
    let list_entry = Entry::App(
        "publishersGroup".into(),
        group.into()
    );

    // commit the entry and return the address
    hdk::commit_entry(&list_entry)
}

pub fn handle_get_groups() -> ZomeApiResult<Vec<Address>> {
    hdk::query("publishersGroup".into(), 0, 0)
}

fn handle_add(publisher_item: Publisher, publisher_addr: HashString) -> ZomeApiResult<Address> {
    // define the entry
    let publisher_entry = Entry::App(
        "publishersItem".into(),
        publisher_item.into()
    );

    let item_addr = hdk::commit_entry(&publisher_entry)?; // commit the list item
    hdk::link_entries(&publisher_addr, &item_addr, "items", "")?; // if successful, link to list address
    Ok(item_addr)
}

fn handle_get_all(publisher_addr: HashString) -> ZomeApiResult<GetPublishersResponse> {

    // load the list entry. Early return error if it cannot load or is wrong type
    let list = hdk::utils::get_as_type::<PublisherGroup>(publisher_addr.clone())?;

    // try and load the list items, filter out errors and collect in a vector
    let list_items = hdk::get_links(&publisher_addr, LinkMatch::Exactly("items"), LinkMatch::Any)?.addresses()
        .iter()
        .map(|item_address| {
            hdk::utils::get_as_type::<Publisher>(item_address.to_owned())
        })
        .filter_map(Result::ok)
        .collect::<Vec<Publisher>>();

    // if this was successful then return the list items
    Ok(GetPublishersResponse{
        name: list.name,
        items: list_items
    })
}