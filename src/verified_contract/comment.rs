use super::Vote;
use crate::str_serializers::*;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, NearSchema, Timestamp};
use std::collections::HashSet;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, NearSchema)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct Comment {
    pub id: u64,
    pub author_id: AccountId,
    #[serde(with = "u64_dec_format")]
    pub timestamp: Timestamp,
    pub content: String,
    pub votes: HashSet<Vote>,
}
