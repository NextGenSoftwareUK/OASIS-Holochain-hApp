use hdk::prelude::*;
use holonet_api_integrity::*;

#[hdk_extern]
pub fn get_all_data(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_data");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllData, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}

// #[hdk_extern]
// pub fn batch_update_avatars(_: ()) -> ExternResult<Vec<Record>> {
    
//     Ok(records)
// }

// #[hdk_extern]
// pub fn add_avatar(_: ()) -> ExternResult<Vec<Record>> {
    
//     Ok(records)
// }

// #[hdk_extern]
// pub fn remove_avatar(_: ()) -> ExternResult<Vec<Record>> {
    
//     Ok(records)
// }