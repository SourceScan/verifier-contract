pub mod github_data;
pub mod like;
pub mod comment;

use std::collections::HashSet;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use like::Like;
use github_data::GithubData;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractData {
    pub cid: String,
    pub lang: String,
    pub entry_point: String,
    pub code_hash: String,
    pub builder_image: String,
    pub github: Option<GithubData>,
    pub likes: HashSet<Like>,
    pub comments: Vec<u64>,
}