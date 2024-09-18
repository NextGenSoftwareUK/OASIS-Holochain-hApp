use hdk::prelude::*;
use oasis_integrity::*;

#[hdk_extern]
pub fn create_holon(holon: Holon) -> ExternResult<Record> {
    let holon_action_hash = create_entry(&EntryTypes::Holon(holon.clone()))?;
    let record = get(holon_action_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Holon"))
            ),
        )?;
    
        let path = Path::from("all_holons");
    
    create_link(
        path.path_entry_hash()?,
        holon_action_hash.clone(),
        LinkTypes::AllHolons,
        (),
    )?;

    let path = Path::from(holon.username);

    create_link(
        path.path_entry_hash()?,
        holon_action_hash.clone(),
        LinkTypes::AllHolonsByUsername,
        (),
    )?;

    Ok(record)
    //Ok(holon_action_hash)
}

#[hdk_extern]
//pub fn get_holon(original_holon_hash: ActionHash, version: i32) -> ExternResult<Option<Record>> {
pub fn get_holon(original_holon_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(original_holon_hash.clone(), LinkTypes::HolonUpdates, None)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_holon_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_holon_hash.clone(),
    };
    get(latest_holon_hash, GetOptions::default())
}

// #[hdk_extern]
// pub fn get_holon_by_username(username: String) -> ExternResult<Option<Record>> {
    
//     //TODO: Need to search/lookup the holon or actionhash which matches the given username...
//     let path = Path::from(username);
//     let links = get_links(AnyLinkableHash::try_from(path.path_entry_hash()?)?, LinkTypes::AllHolonsByUsername, None)?;
    
//     let latest_link = links
//         .into_iter()
//         .max_by(|link_a, link_b| link_a. timestamp.cmp(&link_b.timestamp));
    
//         let latest_holon_hash = match latest_link {
//         Some(link) => ActionHash::from(link.target.clone()),
//         None => path.path_entry_hash()?,
//     };

//     get(latest_holon_hash, GetOptions::default())
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHolonInput {
    pub original_holon_hash: ActionHash,
    pub previous_holon_hash: ActionHash,
    pub updated_holon: Holon,
}
#[hdk_extern]
pub fn update_holon(input: UpdateHolonInput) -> ExternResult<Record> {
    let updated_holon_hash = update_entry(
        input.previous_holon_hash.clone(),
        &input.updated_holon,
    )?;
    create_link(
        input.original_holon_hash.clone(),
        updated_holon_hash.clone(),
        LinkTypes::HolonUpdates,
        (),
    )?;
    let record = get(updated_holon_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Holon"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_holon(original_holon_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_holon_hash)
}
