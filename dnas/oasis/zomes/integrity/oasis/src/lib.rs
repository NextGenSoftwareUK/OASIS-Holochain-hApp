use hdi::prelude::*;
use serde_json::{Map, Value};

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Avatar {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(flatten)] pub fields: Map<String, Value>,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct AvatarDetail {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(flatten)] pub fields: Map<String, Value>,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Holon {
    pub id: String,
    #[serde(default)] pub parent_holon_id: Value,
    #[serde(default)] pub provider_unique_storage_key: Value,
    #[serde(default)] pub meta_data: Value,
    #[serde(default)] pub custom_key: Value,
    #[serde(flatten)] pub fields: Map<String, Value>,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct HyperDriveMutation {
    pub operation_id: String,
    pub avatar_id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub kind: u8,
    pub version_id: String,
    pub payload_json: Option<String>,
}

impl HyperDriveMutation {
    fn validate(&self) -> ValidateCallbackResult {
        if [&self.operation_id, &self.avatar_id, &self.entity_id, &self.entity_type, &self.version_id]
            .iter().any(|value| value.trim().is_empty()) {
            return ValidateCallbackResult::Invalid("HyperDrive identifiers and entity type are required".into());
        }
        if self.kind > 1 {
            return ValidateCallbackResult::Invalid("HyperDrive kind must be 0 (upsert) or 1 (delete)".into());
        }
        if self.kind == 0 && self.payload_json.as_deref().map(str::trim).filter(|v| !v.is_empty()).is_none() {
            return ValidateCallbackResult::Invalid("HyperDrive upserts require a payload".into());
        }
        ValidateCallbackResult::Valid
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Avatar(Avatar), AvatarDetail(AvatarDetail), Holon(Holon), HyperDriveMutation(HyperDriveMutation),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    AvatarById, AvatarByUsername, AvatarByEmail, AllAvatars,
    AvatarDetailById, AvatarDetailByUsername, AvatarDetailByEmail, AllAvatarDetails,
    HolonById, HolonByParentId, HolonByProviderKey, HolonByCustomKey, HolonByMetadata, AllHolons,
    HyperDriveOperations, HyperDriveEntityVersions,
}

#[hdk_extern]
pub fn genesis_self_check(_: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> { Ok(ValidateCallbackResult::Valid) }

pub fn validate_agent_joining(_: AgentPubKey, _: &Option<MembraneProof>) -> ExternResult<ValidateCallbackResult> { Ok(ValidateCallbackResult::Valid) }

fn validate_entry(entry: EntryTypes, updating: bool) -> ValidateCallbackResult {
    match entry {
        EntryTypes::Avatar(ref a) if ["password", "jwt_token", "refresh_token", "refresh_tokens", "reset_token", "verification_token"]
            .iter().any(|key| a.fields.contains_key(*key)) => ValidateCallbackResult::Invalid("Authentication secrets must never be published to the Holochain DHT".into()),
        EntryTypes::Avatar(a) if a.id.trim().is_empty() || a.username.trim().is_empty() => ValidateCallbackResult::Invalid("Avatar id and username are required".into()),
        EntryTypes::AvatarDetail(a) if a.id.trim().is_empty() || a.username.trim().is_empty() => ValidateCallbackResult::Invalid("Avatar detail id and username are required".into()),
        EntryTypes::Holon(h) if h.id.trim().is_empty() => ValidateCallbackResult::Invalid("Holon id is required".into()),
        EntryTypes::HyperDriveMutation(_) if updating => ValidateCallbackResult::Invalid("HyperDrive mutations are immutable".into()),
        EntryTypes::HyperDriveMutation(m) => m.validate(),
        _ => ValidateCallbackResult::Valid,
    }
}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    let result = match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::CreateEntry(OpEntry::CreateEntry { app_entry, .. }) => validate_entry(app_entry, false),
        FlatOp::CreateEntry(OpEntry::UpdateEntry { app_entry, .. }) => validate_entry(app_entry, true),
        FlatOp::Update(OpUpdate::Entry { app_entry, .. }) => validate_entry(app_entry, true),
        FlatOp::CreateRecord(OpRecord::CreateEntry { app_entry, .. }) => validate_entry(app_entry, false),
        FlatOp::CreateRecord(OpRecord::UpdateEntry { app_entry, .. }) => validate_entry(app_entry, true),
        FlatOp::AgentActivity(ref activity) => match activity {
            OpActivity::CreateAgent { action, agent } => {
                let prev = action.prev_action().ok_or_else(|| wasm_error!(WasmErrorInner::Guest("expected prior action".into())))?.clone();
                let previous = must_get_action(prev)?;
                match &previous.action().data {
                    ActionData::AgentValidationPkg(AgentValidationPkgData { membrane_proof, .. }) => validate_agent_joining(agent.clone(), membrane_proof)?,
                    _ => ValidateCallbackResult::Invalid("CreateAgent must follow AgentValidationPkg".into()),
                }
            }
            _ => ValidateCallbackResult::Valid,
        },
        _ => ValidateCallbackResult::Valid,
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_invalid_hyperdrive_upsert() {
        let m = HyperDriveMutation { operation_id: "op".into(), avatar_id: "a".into(), entity_id: "e".into(), entity_type: "holon".into(), kind: 0, version_id: "v".into(), payload_json: None };
        assert!(matches!(m.validate(), ValidateCallbackResult::Invalid(_)));
    }
    #[test]
    fn accepts_hyperdrive_delete_without_payload() {
        let m = HyperDriveMutation { operation_id: "op".into(), avatar_id: "a".into(), entity_id: "e".into(), entity_type: "holon".into(), kind: 1, version_id: "v".into(), payload_json: None };
        assert_eq!(m.validate(), ValidateCallbackResult::Valid);
    }
}
