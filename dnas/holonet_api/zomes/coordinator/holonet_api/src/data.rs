use hdk::prelude::*;
use holonet_api_integrity::*;

#[hdk_extern]
pub fn create_data(data: Data) -> ExternResult<Record> {
    let data_action_hash = create_entry(&EntryTypes::Data(data.clone()))?;
    let record = get(data_action_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created data"))
            ),
        )?;
    
        let path = Path::from("all_datas");
    
    create_link(
        path.path_entry_hash()?,
        data_action_hash.clone(),
        LinkTypes::AllData,
        (),
    )?;

    Ok(record)
    //Ok(data_action_hash)
}

#[hdk_extern]
//pub fn get_data(original_data_hash: ActionHash, version: i32) -> ExternResult<Option<Record>> {
pub fn get_data(original_data_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(original_data_hash.clone(), LinkTypes::DataUpdates, None)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_data_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_data_hash.clone(),
    };
    get(latest_data_hash, GetOptions::default())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDataInput {
    pub original_data_hash: ActionHash,
    pub previous_data_hash: ActionHash,
    pub updated_data: Data,
}
#[hdk_extern]
pub fn update_data(input: UpdateDataInput) -> ExternResult<Record> {
    let updated_data_hash = update_entry(
        input.previous_data_hash.clone(),
        &input.updated_data,
    )?;
    create_link(
        input.original_data_hash.clone(),
        updated_data_hash.clone(),
        LinkTypes::DataUpdates,
        (),
    )?;
    let record = get(updated_data_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Data"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_data(original_data_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_data_hash)
}
