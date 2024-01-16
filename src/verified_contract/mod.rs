pub mod github;
pub mod vote;
pub mod comment;

use std::collections::HashSet;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use vote::Vote;
use github::Github;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VerifiedContract {
    pub cid: String,
    pub lang: String,
    pub entry_point: String,
    pub code_hash: String,
    pub builder_image: String,
    pub github: Option<Github>,
    pub votes: HashSet<Vote>,
    pub comments: Vec<u64>,
}