use super::Vote;
use crate::str_serializers::*;
use near_sdk::{near, AccountId, Timestamp};
use std::collections::HashSet;

#[near(serializers=[borsh, json])]
#[derive(Clone)]
pub struct Comment {
    pub id: u64,
    pub author_id: AccountId,
    #[serde(
        serialize_with = "u64_dec_format::serialize",
        deserialize_with = "u64_dec_format::deserialize"
    )]
    pub timestamp: Timestamp,
    pub content: String,
    pub votes: HashSet<Vote>,
}
