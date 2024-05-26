pub mod comment;
pub mod vote;

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::NearSchema;
use std::collections::HashSet;
use vote::Vote;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, NearSchema)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct VerifiedContract {
    pub cid: String,
    pub lang: String,
    pub code_hash: String,
    pub votes: HashSet<Vote>,
    pub comments: Vec<u64>,
}
