use hdk::prelude::*;
use oasis_integrity::*;

fn operation_path(operation_id: &str) -> Path {
    Path::from(format!("hyperdrive.operations.{}", operation_id))
}

fn entity_path(mutation: &HyperDriveMutation) -> Path {
    Path::from(format!("hyperdrive.entities.{}.{}.{}",
        mutation.avatar_id, mutation.entity_type, mutation.entity_id))
}

/// Atomically appends a replicated mutation and its two immutable indexes to the
/// caller's source chain. Replays return the original record without appending actions.
#[hdk_extern]
pub fn apply_hyperdrive_mutation(mutation: HyperDriveMutation) -> ExternResult<Record> {
    let op_path = operation_path(&mutation.operation_id);
    let existing = get_links(LinkQuery::try_new(op_path.path_entry_hash()?, LinkTypes::HyperDriveOperations)?, GetStrategy::default())?;
    if let Some(link) = existing.into_iter().next() {
        let action_hash = ActionHash::try_from(link.target)
            .map_err(|_| wasm_error!(WasmErrorInner::Guest(
                "HyperDrive operation index target is not an action hash".to_string())))?;
        return get(action_hash, GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Indexed HyperDrive mutation is unavailable".to_string())));
    }

    let entity_index = entity_path(&mutation);
    let action_hash = create_entry(EntryTypes::HyperDriveMutation(mutation))?;
    create_link(op_path.path_entry_hash()?, action_hash.clone(),
        LinkTypes::HyperDriveOperations, ())?;
    create_link(entity_index.path_entry_hash()?, action_hash.clone(),
        LinkTypes::HyperDriveEntityVersions, ())?;
    get(action_hash, GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("New HyperDrive mutation is unavailable".to_string())))
}

#[hdk_extern]
pub fn get_hyperdrive_mutation(operation_id: String) -> ExternResult<Option<Record>> {
    let links = get_links(LinkQuery::try_new(operation_path(&operation_id).path_entry_hash()?,
        LinkTypes::HyperDriveOperations)?, GetStrategy::default())?;
    match links.into_iter().next() {
        Some(link) => {
            let action_hash = ActionHash::try_from(link.target)
                .map_err(|_| wasm_error!(WasmErrorInner::Guest(
                    "HyperDrive operation index target is not an action hash".to_string())))?;
            get(action_hash, GetOptions::default())
        },
        None => Ok(None),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyperDriveEntityInput {
    pub avatar_id: String,
    pub entity_type: String,
    pub entity_id: String,
}

#[hdk_extern]
pub fn get_latest_hyperdrive_entity(input: HyperDriveEntityInput) -> ExternResult<Option<Record>> {
    let mutation = HyperDriveMutation {
        operation_id: String::new(), avatar_id: input.avatar_id,
        entity_id: input.entity_id, entity_type: input.entity_type,
        kind: 0, version_id: String::new(), payload_json: None,
    };
    let latest = get_links(LinkQuery::try_new(entity_path(&mutation).path_entry_hash()?,
        LinkTypes::HyperDriveEntityVersions)?, GetStrategy::default())?.into_iter()
        .max_by(|a, b| a.timestamp.cmp(&b.timestamp));
    match latest {
        Some(link) => {
            let action_hash = ActionHash::try_from(link.target)
                .map_err(|_| wasm_error!(WasmErrorInner::Guest(
                    "HyperDrive entity index target is not an action hash".to_string())))?;
            get(action_hash, GetOptions::default())
        },
        None => Ok(None),
    }
}
