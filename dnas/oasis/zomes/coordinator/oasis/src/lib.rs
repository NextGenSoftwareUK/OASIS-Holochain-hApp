use hdk::prelude::*;
use oasis_integrity::*;
use serde_json::Value;

pub mod hyperdrive;

#[hdk_extern]
pub fn init() -> ExternResult<InitCallbackResult> { Ok(InitCallbackResult::Pass) }

fn index_base(kind: &str, value: &str) -> ExternResult<EntryHash> {
    Path::from(format!("oasis.{kind}.{}", value.replace('.', "_dot_").replace('/', "_slash_"))).path_entry_hash()
}

fn index_record(kind: &str, value: &str, target: ActionHash, link_type: LinkTypes) -> ExternResult<()> {
    create_link(index_base(kind, value)?, target, link_type, ())?;
    Ok(())
}

fn linked_records(kind: &str, value: &str, link_type: LinkTypes) -> ExternResult<Vec<Record>> {
    let links = get_links(LinkQuery::try_new(index_base(kind, value)?, link_type)?, GetStrategy::default())?;
    let mut records = Vec::new();
    for link in links {
        let hash = ActionHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("index target is not an action hash".into())))?;
        if let Some(record) = get(hash, GetOptions::default())? { records.push(record); }
    }
    records.sort_by_key(|record| record.action().timestamp());
    Ok(records)
}

fn latest(kind: &str, value: &str, link_type: LinkTypes) -> ExternResult<Option<Record>> {
    Ok(linked_records(kind, value, link_type)?.pop())
}

fn create_avatar_record(avatar: Avatar) -> ExternResult<Record> {
    let hash = create_entry(EntryTypes::Avatar(avatar.clone()))?;
    index_avatar(&avatar, hash.clone())?;
    get(hash, GetOptions::default())?.ok_or_else(|| wasm_error!(WasmErrorInner::Guest("created avatar unavailable".into())))
}
fn index_avatar(avatar: &Avatar, hash: ActionHash) -> ExternResult<()> {
    index_record("avatar-id", &avatar.id, hash.clone(), LinkTypes::AvatarById)?;
    index_record("avatar-username", &avatar.username, hash.clone(), LinkTypes::AvatarByUsername)?;
    index_record("avatar-email", &avatar.email, hash.clone(), LinkTypes::AvatarByEmail)?;
    index_record("avatars", "all", hash.clone(), LinkTypes::AllAvatars)?;
    Ok(())
}

#[hdk_extern] pub fn create_entry_avatar(avatar: Avatar) -> ExternResult<Record> { create_avatar_record(avatar) }
#[hdk_extern] pub fn create_avatar(avatar: Avatar) -> ExternResult<Record> { create_avatar_record(avatar) }
#[hdk_extern] pub fn get_entry_avatar(hash: ActionHash) -> ExternResult<Option<Record>> { get(hash, GetOptions::default()) }
#[hdk_extern] pub fn get_avatar_by_id(value: String) -> ExternResult<Option<Record>> { latest("avatar-id", &value, LinkTypes::AvatarById) }
#[hdk_extern] pub fn get_avatar_by_username(value: String) -> ExternResult<Option<Record>> { latest("avatar-username", &value, LinkTypes::AvatarByUsername) }
#[hdk_extern] pub fn get_avatar_by_email(value: String) -> ExternResult<Option<Record>> { latest("avatar-email", &value, LinkTypes::AvatarByEmail) }
#[hdk_extern] pub fn get_all_avatars(_: ()) -> ExternResult<Vec<Record>> { linked_records("avatars", "all", LinkTypes::AllAvatars) }

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAvatarInput { pub original_action_hash: ActionHash, pub updated_entry: Avatar }
#[hdk_extern]
pub fn update_entry_avatar(input: UpdateAvatarInput) -> ExternResult<Record> {
    let hash = update_entry(input.original_action_hash, &EntryTypes::Avatar(input.updated_entry.clone()))?;
    index_avatar(&input.updated_entry, hash.clone())?;
    get(hash, GetOptions::default())?.ok_or_else(|| wasm_error!(WasmErrorInner::Guest("updated avatar unavailable".into())))
}

fn delete_latest(kind: &str, value: &str, link_type: LinkTypes) -> ExternResult<ActionHash> {
    let record = latest(kind, value, link_type)?.ok_or_else(|| wasm_error!(WasmErrorInner::Guest("record not found".into())))?;
    delete_entry(record.action_address().clone())
}
#[hdk_extern] pub fn delete_entry_avatar(hash: ActionHash) -> ExternResult<ActionHash> { delete_entry(hash) }
#[hdk_extern] pub fn delete_avatar_by_id(v: String) -> ExternResult<ActionHash> { delete_latest("avatar-id", &v, LinkTypes::AvatarById) }
#[hdk_extern] pub fn delete_avatar_by_username(v: String) -> ExternResult<ActionHash> { delete_latest("avatar-username", &v, LinkTypes::AvatarByUsername) }
#[hdk_extern] pub fn delete_avatar_by_email(v: String) -> ExternResult<ActionHash> { delete_latest("avatar-email", &v, LinkTypes::AvatarByEmail) }

fn create_avatar_detail_record(detail: AvatarDetail) -> ExternResult<Record> {
    let hash = create_entry(EntryTypes::AvatarDetail(detail.clone()))?;
    index_avatar_detail(&detail, hash.clone())?;
    get(hash, GetOptions::default())?.ok_or_else(|| wasm_error!(WasmErrorInner::Guest("created avatar detail unavailable".into())))
}
fn index_avatar_detail(detail: &AvatarDetail, hash: ActionHash) -> ExternResult<()> {
    index_record("detail-id", &detail.id, hash.clone(), LinkTypes::AvatarDetailById)?;
    index_record("detail-username", &detail.username, hash.clone(), LinkTypes::AvatarDetailByUsername)?;
    index_record("detail-email", &detail.email, hash.clone(), LinkTypes::AvatarDetailByEmail)?;
    index_record("details", "all", hash.clone(), LinkTypes::AllAvatarDetails)?;
    Ok(())
}
#[hdk_extern] pub fn create_entry_avatar_detail(v: AvatarDetail) -> ExternResult<Record> { create_avatar_detail_record(v) }
#[hdk_extern] pub fn get_entry_avatar_detail(hash: ActionHash) -> ExternResult<Option<Record>> { get(hash, GetOptions::default()) }
#[hdk_extern] pub fn get_avatar_detail_by_id(v: String) -> ExternResult<Option<Record>> { latest("detail-id", &v, LinkTypes::AvatarDetailById) }
#[hdk_extern] pub fn get_avatar_detail_by_username(v: String) -> ExternResult<Option<Record>> { latest("detail-username", &v, LinkTypes::AvatarDetailByUsername) }
#[hdk_extern] pub fn get_avatar_detail_by_email(v: String) -> ExternResult<Option<Record>> { latest("detail-email", &v, LinkTypes::AvatarDetailByEmail) }
#[hdk_extern] pub fn get_all_avatar_details(_: ()) -> ExternResult<Vec<Record>> { linked_records("details", "all", LinkTypes::AllAvatarDetails) }
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAvatarDetailInput { pub original_action_hash: ActionHash, pub updated_entry: AvatarDetail }
#[hdk_extern] pub fn update_entry_avatar_detail(v: UpdateAvatarDetailInput) -> ExternResult<Record> { let hash = update_entry(v.original_action_hash, &EntryTypes::AvatarDetail(v.updated_entry.clone()))?; index_avatar_detail(&v.updated_entry, hash.clone())?; get(hash, GetOptions::default())?.ok_or_else(|| wasm_error!(WasmErrorInner::Guest("updated avatar detail unavailable".into()))) }
#[hdk_extern] pub fn delete_entry_avatar_detail(hash: ActionHash) -> ExternResult<ActionHash> { delete_entry(hash) }

fn value_key(value: &Value) -> String { value.to_string() }
fn create_holon_record(holon: Holon) -> ExternResult<Record> {
    let hash = create_entry(EntryTypes::Holon(holon.clone()))?;
    index_holon(&holon, hash.clone())?;
    get(hash, GetOptions::default())?.ok_or_else(|| wasm_error!(WasmErrorInner::Guest("created holon unavailable".into())))
}
fn index_holon(holon: &Holon, hash: ActionHash) -> ExternResult<()> {
    index_record("holon-id", &holon.id, hash.clone(), LinkTypes::HolonById)?;
    index_record("holon-parent", &value_key(&holon.parent_holon_id), hash.clone(), LinkTypes::HolonByParentId)?;
    index_record("holon-provider", &value_key(&holon.provider_unique_storage_key), hash.clone(), LinkTypes::HolonByProviderKey)?;
    index_record("holon-custom", &value_key(&holon.custom_key), hash.clone(), LinkTypes::HolonByCustomKey)?;
    index_record("holon-metadata", &value_key(&holon.meta_data), hash.clone(), LinkTypes::HolonByMetadata)?;
    index_record("holons", "all", hash.clone(), LinkTypes::AllHolons)?;
    Ok(())
}
#[hdk_extern] pub fn create_entry_holon(v: Holon) -> ExternResult<Record> { create_holon_record(v) }
#[hdk_extern] pub fn get_entry_holon(hash: ActionHash) -> ExternResult<Option<Record>> { get(hash, GetOptions::default()) }
#[hdk_extern] pub fn get_holon_by_id(v: String) -> ExternResult<Option<Record>> { latest("holon-id", &v, LinkTypes::HolonById) }
#[hdk_extern] pub fn get_holon_by_provider_key(v: String) -> ExternResult<Option<Record>> { latest("holon-provider", &v, LinkTypes::HolonByProviderKey) }
#[hdk_extern] pub fn get_holon_by_custom_key(v: String) -> ExternResult<Option<Record>> { latest("holon-custom", &v, LinkTypes::HolonByCustomKey) }
#[hdk_extern] pub fn get_holon_by_meta_data(v: String) -> ExternResult<Option<Record>> { latest("holon-metadata", &v, LinkTypes::HolonByMetadata) }
#[hdk_extern] pub fn get_holons_for_parent_by_id(v: String) -> ExternResult<Vec<Record>> { linked_records("holon-parent", &format!("\"{v}\""), LinkTypes::HolonByParentId) }
#[hdk_extern] pub fn get_holons_for_parent_by_provider_key(v: String) -> ExternResult<Vec<Record>> { linked_records("holon-provider", &v, LinkTypes::HolonByProviderKey) }
#[hdk_extern] pub fn get_holons_for_parent_by_custom_key(v: String) -> ExternResult<Vec<Record>> { linked_records("holon-custom", &v, LinkTypes::HolonByCustomKey) }
#[hdk_extern] pub fn get_holons_for_parent_by_meta_data(v: String) -> ExternResult<Vec<Record>> { linked_records("holon-metadata", &v, LinkTypes::HolonByMetadata) }
#[hdk_extern] pub fn get_all_holons(_: ()) -> ExternResult<Vec<Record>> { linked_records("holons", "all", LinkTypes::AllHolons) }
#[hdk_extern] pub fn save_all_holons(values: Vec<Holon>) -> ExternResult<Vec<Record>> { values.into_iter().map(create_holon_record).collect() }
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHolonInput { pub original_action_hash: ActionHash, pub updated_entry: Holon }
#[hdk_extern] pub fn update_entry_holon(v: UpdateHolonInput) -> ExternResult<Record> { let hash = update_entry(v.original_action_hash, &EntryTypes::Holon(v.updated_entry.clone()))?; index_holon(&v.updated_entry, hash.clone())?; get(hash, GetOptions::default())?.ok_or_else(|| wasm_error!(WasmErrorInner::Guest("updated holon unavailable".into()))) }
#[hdk_extern] pub fn delete_entry_holon(hash: ActionHash) -> ExternResult<ActionHash> { delete_entry(hash) }
#[hdk_extern] pub fn delete_holon_by_id(v: String) -> ExternResult<ActionHash> { delete_latest("holon-id", &v, LinkTypes::HolonById) }
#[hdk_extern] pub fn delete_holon_by_provider_key(v: String) -> ExternResult<ActionHash> { delete_latest("holon-provider", &v, LinkTypes::HolonByProviderKey) }
#[hdk_extern] pub fn delete_holon_by_custom_key(v: String) -> ExternResult<ActionHash> { delete_latest("holon-custom", &v, LinkTypes::HolonByCustomKey) }
#[hdk_extern] pub fn delete_holon_by_meta_data(v: String) -> ExternResult<ActionHash> { delete_latest("holon-metadata", &v, LinkTypes::HolonByMetadata) }
