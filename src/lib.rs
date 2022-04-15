use bdk::bitcoin::Network;
use bdk::blockchain::EsploraBlockchain;
use bdk::database::MemoryDatabase;
use bdk::wallet::AddressIndex;
use bdk::{SyncOptions, Wallet};
use std::rc::Rc;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[cfg(feature = "web-sys")]
use web_sys::console;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//#[wasm_bindgen]
//pub fn init() {
//    console_log::init_with_level(log::Level::Debug).unwrap();
//    utils::set_panic_hook();
//
//    info!("Initialization completed");
//}

#[wasm_bindgen]
pub struct WalletWrapper {
    wallet: Rc<Wallet<MemoryDatabase>>,
    blockchain: Rc<EsploraBlockchain>,
}

#[wasm_bindgen]
impl WalletWrapper {
    #[wasm_bindgen(constructor)]
    pub async fn new(
        network: String,
        descriptor: String,
        change_descriptor: Option<String>,
        esplora: String,
        stop_gap: usize,
    ) -> Result<WalletWrapper, String> {
        let network = match network.as_str() {
            "regtest" => Network::Regtest,
            "testnet" | _ => Network::Testnet,
        };

        let blockchain = EsploraBlockchain::new(&esplora, stop_gap);
        let wallet = Wallet::new(
            descriptor.as_str(),
            change_descriptor.as_ref().map(|x| x.as_str()),
            network,
            MemoryDatabase::new(),
        )
        .map_err(|e| format!("{:?}", e))?;

        Ok(WalletWrapper {
            wallet: Rc::new(wallet),
            blockchain: Rc::new(blockchain),
        })
    }

    pub fn sync(&self) -> Promise {
        let wallet = Rc::clone(&self.wallet);
        let blockchain = Rc::clone(&self.blockchain);
        future_to_promise(async move {
            #[cfg(feature = "web-sys")]
            console::log_1(&"before sync".into());
            wallet
                .as_ref()
                .sync(blockchain.as_ref(), SyncOptions::default())
                .await
                .map_err(|e| format!("{:?}", e))?;
            #[cfg(feature = "web-sys")]
            console::log_1(&"after sync".into());
            Ok("done".into())
        })
    }

    #[wasm_bindgen]
    pub fn balance(&self) -> Result<u64, String> {
        let balance = self.wallet.get_balance().map_err(|e| format!("{:?}", e))?;
        Ok(balance)
    }

    #[wasm_bindgen]
    pub fn get_new_address(&self) -> Result<String, String> {
        let new_address = self
            .wallet
            .get_address(AddressIndex::New)
            .map_err(|e| format!("{:?}", e))?
            .address
            .to_string();
        Ok(new_address)
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, bdk-wasm!");
}
