mod contract_data;

use contract_data::ContractData;
use contract_data::GithubData;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, require, log};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SourceScan {
    owner_id: AccountId,
    contracts: UnorderedMap<AccountId, ContractData>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    SourceScanRecords,
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
            contracts: UnorderedMap::new(StorageKey::SourceScanRecords),
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

    pub fn set_contract(&mut self, account_id: AccountId, cid: String, code_hash: String, lang: String, entry_point: String, builder_image: String, github: Option<GithubData>) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner can call this method");

        self.contracts.insert(&account_id, &ContractData {
            cid: cid,
            code_hash: code_hash,
            lang: lang,
            entry_point: entry_point,
            builder_image: builder_image,
            github: match github {
                Some(github_data) => Some(GithubData {
                    owner: github_data.owner.clone(),
                    repo: github_data.repo.clone(),
                    sha: github_data.sha.clone(),
                }),
                None => None,
            },
        });

        log!("Contract {} added", env::predecessor_account_id());
    }

    pub fn search(&self, key: String, from_index: usize, limit: usize) -> (Vec<(AccountId, ContractData)>, u64) {
        let mut result: Vec<(AccountId, ContractData)> = Vec::new();

        for (k, v) in self.contracts.iter()
        {            
            if k.as_str().to_lowercase().replace(".testnet", "").replace(".near", "").contains(&key.to_lowercase()) {
                result.push((k, v));
            }
        }
        
        let pages: u64 = self.get_pages(result.len() as u64, limit as u64);
        let filtered: Vec<(AccountId, ContractData)> = result
        .into_iter()
        .skip(from_index)
        .take(limit)
        .collect();

        return (filtered, pages);
    }

    pub fn purge_contract(&mut self, account_id: AccountId) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner can call this method");

        self.contracts.remove(&account_id);

        log!("Contract {} removed", account_id);
    }

    pub fn get_contract(&self, account_id: AccountId) -> Option<ContractData> {       
        return self.contracts.get(&account_id);
    }

    pub fn get_contracts(&self, from_index: usize, limit: usize) -> (Vec<(AccountId, ContractData)>, u64) {
        let filtered:Vec<(AccountId, ContractData)> = self.contracts
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
        let github_data = GithubData {
            owner: "owner".to_string(),
            repo: "repo".to_string(),
            sha: "sha".to_string(),
        };

        contract.set_contract(
            accounts(1), 
            "cid".to_string(), 
            "code_hash".to_string(), 
            "lang".to_string(), 
            "entry_point".to_string(), 
            "builder_image".to_string(), 
            Some(github_data)
        );

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

        // Setup: Add a contract
        let github_data = GithubData {
            owner: "owner".to_string(),
            repo: "repo".to_string(),
            sha: "sha".to_string(),
        };
        contract.set_contract(
            accounts(1), 
            "cid".to_string(), 
            "code_hash".to_string(), 
            "lang".to_string(), 
            "entry_point".to_string(), 
            "builder_image".to_string(), 
            Some(github_data)
        );

        // Action: Purge the contract
        contract.purge_contract(accounts(1));

        // Verification: Ensure contract is removed
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
        // Setup: Add multiple contracts
        for i in 1..4 {
            contract.set_contract(
                accounts(i), 
                format!("cid_{}", i), 
                "code_hash".to_string(), 
                "lang".to_string(), 
                "entry_point".to_string(), 
                "builder_image".to_string(), 
                None
            );
        }

        // Action: Retrieve contracts
        let (contracts, total_pages) = contract.get_contracts(0, 2);

        // Verification: Check the retrieved contracts and pagination
        assert_eq!(contracts.len(), 2);
        assert_eq!(total_pages, 2); // As we have 3 contracts and limit is 2
    }

    #[test]
    fn search_contracts() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = SourceScan::new();
        // Setup: Add contracts with varying account_ids
        contract.set_contract(
            "account1.testnet".parse().unwrap(), 
            "cid1".to_string(), 
            "code_hash1".to_string(), 
            "lang1".to_string(), 
            "entry_point1".to_string(), 
            "builder_image1".to_string(), 
            None
        );
        contract.set_contract(
            "account2.testnet".parse().unwrap(), 
            "cid2".to_string(), 
            "code_hash2".to_string(), 
            "lang2".to_string(), 
            "entry_point2".to_string(), 
            "builder_image2".to_string(), 
            None
        );

        // Action: Search for contracts
        let (search_results, _) = contract.search("account1".to_string(), 0, 10);

        // Verification: Check if the correct contract is retrieved
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].0, "account1.testnet".parse().unwrap());
        assert_eq!(search_results[0].1.cid, "cid1");
    }
}
