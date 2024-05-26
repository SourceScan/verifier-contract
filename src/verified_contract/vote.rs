use crate::str_serializers::*;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, NearSchema, Timestamp};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone,
)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub enum VoteType {
    Upvote,
    Downvote,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Eq, Ord, Clone, NearSchema)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
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

impl PartialOrd for Vote {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}

impl Ord for VoteType {
    fn cmp(&self, other: &Self) -> Ordering {
        use VoteType::*;
        match (self, other) {
            (Upvote, Upvote) => Ordering::Equal,
            (Downvote, Downvote) => Ordering::Equal,
            (Upvote, Downvote) => Ordering::Less,
            (Downvote, Upvote) => Ordering::Greater,
        }
    }
}
