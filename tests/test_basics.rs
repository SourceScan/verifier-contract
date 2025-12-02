use near_workspaces::AccountId;
use serde_json::json;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let owner_account = sandbox.dev_create_account().await?;
    let user_account = sandbox.dev_create_account().await?;

    let init_outcome = owner_account.call(contract.id(), "new").transact().await?;
    assert!(init_outcome.is_success());

    let set_owner_outcome = owner_account
        .call(contract.id(), "set_owner")
        .args_json(json!({ "owner_id": user_account.id() }))
        .transact()
        .await?;
    assert!(set_owner_outcome.is_success());

    let owner_result: AccountId = contract
        .view("get_owner")
        .await?
        .json()?;
    assert_eq!(&owner_result, user_account.id());

    let github_data = json!({
        "owner": "owner",
        "repo": "repo",
        "sha": "sha"
    });
    
    let set_contract_outcome = user_account
    .call(contract.id(), "set_contract")
    .args_json(json!({
        "account_id": user_account.id(),
        "cid": "cid1",
        "code_hash": "hash1",
        "lang": "Rust",
        "entry_point": "main",
        "builder_image": "rust:latest",
        "github": github_data
    }))
    .transact()
    .await?;
    assert!(set_contract_outcome.is_success());

    Ok(())
}
