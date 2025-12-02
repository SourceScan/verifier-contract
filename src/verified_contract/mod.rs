pub mod comment;
pub mod vote;

use near_sdk::near;
use std::collections::HashSet;
use vote::Vote;

#[near(serializers=[borsh, json])]
pub struct VerifiedContract {
    pub cid: String,
    pub lang: String,
    pub code_hash: String,
    pub block_height: u64,
    pub votes: HashSet<Vote>,
    pub comments: Vec<u64>,
}
