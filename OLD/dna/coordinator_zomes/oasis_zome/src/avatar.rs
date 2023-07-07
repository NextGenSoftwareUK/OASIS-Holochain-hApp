use hdk::prelude::*;
use oasis_zome_integrity::Avatar;
use oasis_zome_integrity::EntryTypes;

#[hdk_extern]
pub fn get_avatar(action_hash: ActionHash) -> ExternResult<Option<Record>> {
  get(action_hash, GetOptions::default())
}


#[hdk_extern]
pub fn create_avatar(avatar: Avatar) -> ExternResult<ActionHash> {
  create_entry(&EntryTypes::Avatar(avatar.clone()))
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAvatarInput {
  original_action_hash: ActionHash,
  updated_avatar: Avatar
}

#[hdk_extern]
pub fn update_avatar(input: UpdateAvatarInput) -> ExternResult<ActionHash> {
  update_entry(input.original_action_hash, &input.updated_avatar)
}


#[hdk_extern]
pub fn delete_avatar(action_hash: ActionHash) -> ExternResult<ActionHash> {
  delete_entry(action_hash)
}

