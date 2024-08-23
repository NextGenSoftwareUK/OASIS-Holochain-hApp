use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Avatar {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub title: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_type: String,
    pub accept_terms: bool,
    pub is_verified: bool,
    pub jwt_token: String,
    pub password_reset: String,
    pub refresh_token: String,
    pub refresh_tokens: String,
    pub reset_token: String,
    pub reset_token_expires: String,
    pub verification_token: String,
    pub verified: String,
    pub last_beamed_in: String,
    pub last_beamed_out: String,
    pub is_beamed_in: bool,
    pub name: String,
    pub description: String,
    pub holon_type: String,
    pub provider_unique_storage_key: String,
    pub previous_version_provider_unique_storage_key: String,
    pub provider_wallets: String,
    pub provider_username: String,
    pub provider_meta_data: String,
    pub meta_data: String,
    pub version: i32,
    pub version_id: String,
    pub previous_version_id: String,
    pub is_active: bool,
    pub created_provider_type: String,
    pub created_oasis_type: String,
    pub children: String,
    pub custom_key: String,
    pub instance_saved_on_provider_type: String,
    pub parent_holon_id: String,
    pub child_id_list_cache: String,
    pub all_child_id_list_cache: String,
    pub created_date: String,
    pub created_by: String,
    pub modified_date: String,
    pub modified_by: String,
    pub deleted_date: String,
    pub deleted_by: String,
    // pub created_date: Timestamp,
    // pub created_by: AgentPubKey,
    // pub modified_date: Timestamp,
    // pub modified_by: AgentPubKey,
    // pub deleted_date: Timestamp,
    // pub deleted_by: AgentPubKey,
    //pub is_active: bool,
    //pub version: i32,
    pub state: i32
}
pub fn validate_create_avatar(
    _action: EntryCreationAction,
    _avatar: Avatar,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_avatar(
    _action: Update,
    _avatar: Avatar,
    _original_action: EntryCreationAction,
    _original_avatar: Avatar,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_avatar(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_avatar: Avatar,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_avatar_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _avatar: crate::Avatar = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _avatar: crate::Avatar = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_avatar_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AvatarUpdates links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_avatars(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _avatar: crate::Avatar = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_all_avatars_by_username(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _avatar: crate::Avatar = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_avatars(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllAvatars links cannot be deleted"),
        ),
    )
}
