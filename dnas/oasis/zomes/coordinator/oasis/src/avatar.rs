use hdk::prelude::*;
use oasis_integrity::*;
#[hdk_extern]
pub fn create_avatar(avatar: Avatar) -> ExternResult<Record> {
    let avatar_action_hash = create_entry(&EntryTypes::Avatar(avatar.clone()))?;
    let record = get(avatar_action_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Avatar"))
            ),
        )?;
    
        let path = Path::from("all_avatars");
    
    create_link(
        path.path_entry_hash()?,
        avatar_action_hash.clone(),
        LinkTypes::AllAvatars,
        (),
    )?;

    create_link(
        avatar.username,
        avatar_action_hash.clone(),
        LinkTypes::AllAvatarsByUsername,
        (),
    )?;

    Ok(record)
    //Ok(avatar_action_hash)
}

#[hdk_extern]
//pub fn get_avatar(original_avatar_hash: ActionHash, version: i32) -> ExternResult<Option<Record>> {
pub fn get_avatar(original_avatar_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(original_avatar_hash.clone(), LinkTypes::AvatarUpdates, None)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_avatar_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_avatar_hash.clone(),
    };
    get(latest_avatar_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_avatar_by_username(username: String) -> ExternResult<Option<Record>> {
    
    //TODO: Need to search/lookup the avatar or actionhash which matches the given username...
    
    let links = get_links(username, LinkTypes::AllAvatarsByUsername, None)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_avatar_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => username,
    };
    get(latest_avatar_hash, GetOptions::default())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAvatarInput {
    pub original_avatar_hash: ActionHash,
    pub previous_avatar_hash: ActionHash,
    pub updated_avatar: Avatar,
}
#[hdk_extern]
pub fn update_avatar(input: UpdateAvatarInput) -> ExternResult<Record> {
    let updated_avatar_hash = update_entry(
        input.previous_avatar_hash.clone(),
        &input.updated_avatar,
    )?;
    create_link(
        input.original_avatar_hash.clone(),
        updated_avatar_hash.clone(),
        LinkTypes::AvatarUpdates,
        (),
    )?;
    let record = get(updated_avatar_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Avatar"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_avatar(original_avatar_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_avatar_hash)
}
