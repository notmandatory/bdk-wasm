//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use bdk_wasm::WalletWrapper;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

async fn test_wallet() -> Result<WalletWrapper, String> {
    let network = "testnet".to_string();
    let descriptor = "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)".to_string();
    let change_descriptor = Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)".to_string());
    let esplora = "https://blockstream.info/testnet/api".to_string();
    WalletWrapper::new(network, descriptor, change_descriptor, esplora, 20).await
}

#[wasm_bindgen_test]
async fn test_sync() {
    let wallet = test_wallet().await.expect("wallet");
    let promise = wallet.sync();
    let result = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .expect("result");
    assert_eq!(result.as_string().unwrap(), "done".to_string());

    let balance = wallet.balance().expect("balance");
    assert!(balance > 0);
}

#[wasm_bindgen_test]
async fn test_balance() {
    let wallet = test_wallet().await.expect("wallet");
    let balance = wallet.balance().expect("balance");
    assert_eq!(balance, 0);
}

#[wasm_bindgen_test]
async fn test_new_address() {
    let wallet = test_wallet().await.expect("wallet");
    let new_address = wallet.get_new_address().expect("new_address");
    assert_eq!(
        new_address,
        "tb1qzg4mckdh50nwdm9hkzq06528rsu73hjxxzem3e".to_string()
    );
}
