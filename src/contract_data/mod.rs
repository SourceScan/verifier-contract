use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GithubData {
    pub owner: String,
    pub repo: String,
    pub sha: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractData {
    pub cid: String,
    pub lang: String,
    pub entry_point: String,
    pub code_hash: String,
    pub builder_image: String,
    pub github: Option<GithubData>,
}