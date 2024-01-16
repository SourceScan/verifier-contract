use crate::str_serializers::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Timestamp};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteType {
    Upvote,
    Downvote,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Vote {
    pub author_id: AccountId,
    #[serde(with = "u64_dec_format")]
    pub timestamp: Timestamp,
    pub vote_type: VoteType, 
}

impl Hash for Vote {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only use the author_id for hashing
        self.author_id.hash(state);
    }
}

impl PartialEq for Vote {
    fn eq(&self, other: &Self) -> bool {
        // Votes are equal if they have the same author_id
        self.author_id == other.author_id
    }
}

impl Eq for Vote {}

impl PartialOrd for Vote {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}



