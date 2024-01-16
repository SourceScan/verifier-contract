pub mod str_serializers;
pub mod verified_contract;

use verified_contract::VerifiedContract;
use verified_contract::comment::Comment;
use verified_contract::github::Github;
use verified_contract::vote::{VoteType, Vote};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, require, log};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SourceScan {
    owner_id: AccountId,
    contracts: UnorderedMap<AccountId, VerifiedContract>,
    comments: Vector<Comment>
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    VerifiedContracts,
    Comments,
}

impl Default for SourceScan {
    fn default() -> Self {
        panic!("SourceScan should be initialized before usage")
    }   
}

#[near_bindgen]
impl SourceScan {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        Self {
            owner_id: env::predecessor_account_id(),
            contracts: UnorderedMap::new(StorageKey::VerifiedContracts),
            comments: Vector::new(StorageKey::Comments),
        }
    }

    pub fn set_owner(&mut self, owner_id: AccountId) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner can call this method");

        self.owner_id = owner_id;

        log!("Owner changed to {}", self.owner_id)
    }

    pub fn get_owner(&self) -> AccountId {
        return self.owner_id.clone();
    }

    pub fn set_contract(&mut self, account_id: AccountId, cid: String, code_hash: String, lang: String, entry_point: String, builder_image: String, github: Option<Github>) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner can call this method");
    
        let existing_contract: Option<VerifiedContract> = self.contracts.get(&account_id);
    
        self.contracts.insert(&account_id, &VerifiedContract {
            cid,
            code_hash,
            lang,
            entry_point,
            builder_image,
            votes: existing_contract.as_ref().map_or(Default::default(), |c| c.votes.clone()),
            comments: existing_contract.as_ref().map_or(Default::default(), |c| c.comments.clone()),
            github: match github {
                Some(github_data) => Some(Github {
                    owner: github_data.owner,
                    repo: github_data.repo,
                    sha: github_data.sha,
                }),
                None => None,
            },
        });
    
        let action = if existing_contract.is_some() { "updated" } else { "added" };
        log!("Contract {} {}", account_id, action);
    }

    pub fn purge_contract(&mut self, account_id: AccountId) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner can call this method");

        self.contracts.remove(&account_id);

        log!("Contract {} removed", account_id);
    }

    pub fn get_contract(&self, account_id: AccountId) -> Option<VerifiedContract> {       
        return self.contracts.get(&account_id);
    }

    pub fn search(&self, key: String, from_index: usize, limit: usize) -> (Vec<(AccountId, VerifiedContract)>, u64) {
        let mut result: Vec<(AccountId, VerifiedContract)> = Vec::new();

        for (k, v) in self.contracts.iter()
        {            
            if k.as_str().to_lowercase().replace(".testnet", "").replace(".near", "").contains(&key.to_lowercase()) {
                result.push((k, v));
            }
        }
        
        let pages: u64 = self.get_pages(result.len() as u64, limit as u64);
        let filtered: Vec<(AccountId, VerifiedContract)> = result
        .into_iter()
        .skip(from_index)
        .take(limit)
        .collect();

        return (filtered, pages);
    }

    pub fn get_contracts(&self, from_index: usize, limit: usize) -> (Vec<(AccountId, VerifiedContract)>, u64) {
        let filtered:Vec<(AccountId, VerifiedContract)> = self.contracts
        .iter()
        .skip(from_index)
        .take(limit)
        .collect();

        let pages: u64 = self.get_pages(self.contracts.len(), limit as u64);

        return (filtered, pages);
    }

    fn get_pages (&self, len: u64, limit: u64) -> u64 {
        return (len + limit - 1) / limit;
    }

    pub fn add_vote(&mut self, account_id: AccountId, is_upvote: bool) {
        let mut contract: VerifiedContract = self
            .contracts
            .get(&account_id)
            .unwrap_or_else(|| panic!("Contract {} not found", account_id))
            .into();
    
        let author_id = env::predecessor_account_id();
        let current_timestamp = env::block_timestamp();
    
        let vote_type = if is_upvote {
            VoteType::Upvote
        } else {
            VoteType::Downvote
        };
    
        let new_vote = Vote {
            author_id: author_id.clone(),
            timestamp: current_timestamp,
            vote_type: vote_type,
        };
    
        if let Some(mut existing_vote) = contract.votes.take(&new_vote) {
            existing_vote.vote_type = vote_type;
            existing_vote.timestamp = current_timestamp;
            contract.votes.insert(existing_vote);
        } else {
            // If not, insert the new vote
            contract.votes.insert(new_vote);
        }
    
        self.contracts.insert(&account_id, &contract);
        log!("Vote updated for contract {}", account_id);
    }

    pub fn add_comment(&mut self, account_id: AccountId, content: String) {
        let mut contract: VerifiedContract = self
            .contracts
            .get(&account_id)
            .unwrap_or_else(|| panic!("Contract {} not found", account_id))
            .into();
    
        let author_id = env::predecessor_account_id();
        let current_timestamp = env::block_timestamp();
    
        let new_comment = Comment {
            id: self.comments.len() as u64,
            author_id: author_id.clone(),
            timestamp: current_timestamp,
            content: content,
            votes: Default::default(),
        };
    
        contract.comments.push(new_comment.id);
        self.comments.push(&new_comment);
        self.contracts.insert(&account_id, &contract);
        log!("Comment added for contract {}", account_id);
    }

    pub fn get_comments(&self, account_id: AccountId) -> Vec<Comment> {
        let contract: VerifiedContract = self
            .contracts
            .get(&account_id)
            .unwrap_or_else(|| panic!("Contract {} not found", account_id))
            .into();
    
        let mut comments: Vec<Comment> = Vec::new();
    
        for comment_id in contract.comments {
            comments.push(self.comments.get(comment_id).unwrap());
        }
    
        return comments;
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // Helper function to set up the testing environment
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    // Helper function to add a contract
    fn add_contract(contract: &mut SourceScan, account_id: AccountId, with_github: bool) {
        let github_data = if with_github {
            Some(Github {
                owner: "owner".to_string(),
                repo: "repo".to_string(),
                sha: "sha".to_string(),
            })
        } else {
            None
        };

        contract.set_contract(
            account_id, 
            "cid".to_string(), 
            "code_hash".to_string(), 
            "lang".to_string(), 
            "entry_point".to_string(), 
            "builder_image".to_string(), 
            github_data
        );
    }

    #[test]
    #[should_panic(expected = "SourceScan should be initialized before usage")]
    fn default_constructor() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = SourceScan::default(); 
        contract.get_owner(); // This should panic
    }

    #[test]
    fn init_constructor() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = SourceScan::new();
        assert_eq!(contract.owner_id, accounts(0));
    }

    #[test]
    fn set_and_get_owner() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = SourceScan::new();
        contract.set_owner(accounts(1));
        assert_eq!(contract.get_owner(), accounts(1));
    }

    #[test]
    #[should_panic(expected = "Only owner can call this method")]
    fn set_owner_unauthorized() {
        let context = get_context(accounts(1));
        testing_env!(context.build());

        let mut contract = SourceScan::new();
        contract.set_owner(accounts(2));
        contract.set_owner(accounts(3)); // This should panic
    }

    #[test]
    fn set_and_get_contract() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = SourceScan::new();

        add_contract(&mut contract, accounts(1), true);

        let contract_data = contract.get_contract(accounts(1)).unwrap();
        assert_eq!(contract_data.cid, "cid");
        assert_eq!(contract_data.code_hash, "code_hash");
        assert_eq!(contract_data.lang, "lang");
        assert_eq!(contract_data.entry_point, "entry_point");
        assert_eq!(contract_data.builder_image, "builder_image");
        assert!(contract_data.github.is_some());
    }

    #[test]
    fn purge_and_verify_contract() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = SourceScan::new();

        add_contract(&mut contract, accounts(1), true);

        contract.purge_contract(accounts(1));

        assert!(contract.get_contract(accounts(1)).is_none());
    }

    #[test]
    #[should_panic(expected = "Only owner can call this method")]
    fn purge_contract_unauthorized() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut contract = SourceScan::new();
        contract.set_owner(accounts(2));
        contract.purge_contract(accounts(2));
    }

    #[test]
    fn list_and_verify_contracts() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = SourceScan::new();

        for i in 1..4 {
            add_contract(&mut contract, accounts(i), false);
        }

        let (contracts, total_pages) = contract.get_contracts(0, 2);

        assert_eq!(contracts.len(), 2);
        assert_eq!(total_pages, 2);
    }

    #[test]
    fn search_contracts() {
        let context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = SourceScan::new();

        // Setup: Add contracts with varying account_ids using the helper function
        add_contract(&mut contract, "account1.testnet".parse().unwrap(), false);
        add_contract(&mut contract, "account2.testnet".parse().unwrap(), false);

        // Action: Search for contracts
        let (search_results, _) = contract.search("account1".to_string(), 0, 10);

        // Verification: Check if the correct contract is retrieved
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].0, "account1.testnet".parse().unwrap());
    }

    #[test]
    fn test_vote_functionality() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = SourceScan::new();
        add_contract(&mut contract, accounts(1), false);

        // Upvote the contract
        contract.add_vote(accounts(1), true);

        let contract_data = contract.get_contract(accounts(1)).unwrap();
        assert_eq!(contract_data.votes.len(), 1);
        assert!(matches!(contract_data.votes.iter().next().unwrap().vote_type, VoteType::Upvote));

        // Change to downvote
        contract.add_vote(accounts(1), false);

        let contract_data = contract.get_contract(accounts(1)).unwrap();
        assert!(matches!(contract_data.votes.iter().next().unwrap().vote_type, VoteType::Downvote));
    }

    #[test]
    fn test_contract_update() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = SourceScan::new();
        add_contract(&mut contract, accounts(1), false);

        // Upvote the contract
        contract.add_vote(accounts(1), true);

        // Update the contract
        add_contract(&mut contract, accounts(1), true);

        let contract_data = contract.get_contract(accounts(1)).unwrap();
        assert!(matches!(contract_data.votes.iter().next().unwrap().vote_type, VoteType::Upvote));
        assert!(contract_data.github.is_some());
        assert_eq!(contract_data.github.unwrap().owner, "owner");
    }

    #[test]
    fn test_add_comment() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = SourceScan::new();
        add_contract(&mut contract, accounts(1), false);

        contract.add_comment(accounts(1), "Sample comment".to_string());

        let comments = contract.get_comments(accounts(1));
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].content, "Sample comment");
    }

    #[test]
    fn test_get_comments() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = SourceScan::new();
        add_contract(&mut contract, accounts(1), false);

        contract.add_comment(accounts(1), "First comment".to_string());
        contract.add_comment(accounts(1), "Second comment".to_string());

        let comments = contract.get_comments(accounts(1));
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].content, "First comment");
        assert_eq!(comments[1].content, "Second comment");
    }
}
