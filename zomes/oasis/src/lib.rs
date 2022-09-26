use hdk::prelude::*;
//use hdi::prelude::*;

// Old Dev Camp 8 Way
// #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
// #[hdk_entry(id = "avatar", visibility = "public")]
// //#[derive(Clone)]
// pub struct Avatar {
//     pub id: i32,
//     pub first_name: String,
//     pub last_name: String,
//     pub email: String,
//     pub dob: String
// }

// New Scaffolding Way
#[hdk_entry_helper] //Presume this wraps up #[hdk_entry(id = "avatar", visibility = "public")] ?
#[derive(Clone)] // Why don't we need Serialize, Deserialize, SerializedBytes, Debug anymore?
pub struct Avatar {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub dob: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEntryAvatarInput {
  original_action_hash: ActionHash,
  updated_entry_avatar: Avatar
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
#[entry_def()]
Avatar(Avatar),
}

// data we want back from holochain
// #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
// pub struct AvatarInput {
//     pub id: i32,
// }

// #[hdk_extern]
// pub fn get_entry_avatar_old(input: AvatarInput) -> ExternResult<Avatar> {
//     Ok(Avatar {
//       id: input.id,
//       first_name: "David".to_string(),
//       last_name: "Ellams".to_string(),
//       email: "david@nextgensoftware.co.uk".to_string()
//     })
// }

// Dev Camp 8 Way of doing it.
// #[hdk_extern]
// //pub fn create_entry_avatar_old(avatar: Avatar) -> ExternResult<HeaderHash> 
// pub fn create_entry_avatar_old(avatar: Avatar) -> ExternResult<ActionHash> 
// {
//     create_entry(avatar.clone())?;
//     let avatar_entry_hash = hash_entry(avatar)?;
//     Ok(avatar_entry_hash)
// }

// Scaffolding RAD Tool Way.
#[hdk_extern]
pub fn create_entry_avatar(avatar: Avatar) -> ExternResult<ActionHash> {
  create_entry(&EntryTypes::Avatar(avatar.clone())) //Why do we need to use the EntryTypes enum when we never use to?
}

#[hdk_extern]
pub fn get_entry_avatar(action_hash: ActionHash) -> ExternResult<Option<Record>> {
  get(action_hash, GetOptions::default())
}

#[hdk_extern]
pub fn update_entry_avatar(input: UpdateEntryAvatarInput) -> ExternResult<ActionHash> {
  update_entry(input.original_action_hash, &input.updated_entry_avatar)
}

// #[hdk_extern]
// pub fn update_entry_avatar2(original_action_hash: ActionHash, updated_entry_avatar Avatar) -> ExternResult<ActionHash> {
//   update_entry(original_action_hash, &updated_entry_avatar)
// }

#[hdk_extern]
pub fn delete_entry_avatar(action_hash: ActionHash) -> ExternResult<ActionHash> {
  delete_entry(action_hash)
}