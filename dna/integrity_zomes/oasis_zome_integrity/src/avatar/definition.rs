use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Avatar {
  pub title: String,
  pub content: String,
  pub first_name: String,
  pub last_name: String,
}