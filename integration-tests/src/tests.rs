use std::{env, fs};
use near_sdk::json_types::{U128};
use near_units::parse_near;
use serde_json::json;
use workspaces::{Account, Contract};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hellonear_wasm = include_bytes!("../../contract/dist/hello_near.wasm");
    let promise_wasm = include_bytes!("../../contract/dist/promise.wasm");

    let worker = workspaces::sandbox().await?;
    let hello_near = worker.dev_deploy(hellonear_wasm).await?;
    let promise_contract = worker.dev_deploy(promise_wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount( "alice")
        .initial_balance(parse_near!("5 N"))
        .transact()
        .await?
        .into_result()?;

    let init_hello_near = alice.call(hello_near.id(), "init")
    .args_json(json!({
        "_message":"Hello",
        "_account_id_promise":promise_contract.id()})
    )
    .max_gas()
    .transact()
    .await?
    .into_result();


    // begin tests
    test_default_message(&alice, &hello_near).await?;
    test_changes_message(&alice, &hello_near).await?;
    test_set_balance(&alice, &hello_near, &promise_contract).await?;
    Ok(())
}

async fn test_default_message(
    user: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    let message: String = user
        .call( contract.id(), "get_greeting")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(message, "Hello".to_string());
    println!("      Passed ✅ gets default message");
    Ok(())
}

async fn test_changes_message(
    user: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    user.call(contract.id(), "set_greeting")
        .args_json(json!({"message": "Howdy"}))
        .transact()
        .await?
        .into_result()?;

    let message: String = user
        .call(contract.id(), "get_greeting")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(message, "Howdy".to_string());
    println!("      Passed ✅ changes message");
    Ok(())
}

async fn test_set_balance(
    user: &Account, 
    caller_contract: &Contract, 
    receiver_contract: &Contract
) -> anyhow::Result<()> {
    println!("      Working Set Balance");

    user.call(caller_contract.id(), "promise_set_balance")
        .args_json(json!({"_balance":"5000"}))
        .max_gas()
        .transact()
        .await?
        .into_result()?;

    println!("      Call promise success");

    let balance = receiver_contract
    .call("get_balance")
    .view()
    .await?
    .json::<U128>()?;

    println!("      View balance success");

    // println!("Balance changed: {}", balance.to_string().clone());
    assert_eq!(5000, balance.0);
    println!("      Passed ✅ Cross Contract test success");
    Ok(())
}