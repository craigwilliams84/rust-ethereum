extern crate web3;
extern crate ethereum_tx_sign;
extern crate ethereum_types;
extern crate hex;

use web3::futures::Future;
use web3::types::Bytes;
use ethereum_tx_sign::RawTransaction;
use ethereum_types::{H160,H256,U256};

fn main() {
    let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();

    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().wait().unwrap();

    let balance_before = web3.eth().balance(accounts[1], None).wait().unwrap();

    let nonce = web3.eth().transaction_count(accounts[0], None).wait().unwrap();

    let tx = RawTransaction {
        nonce: convert_u256(nonce),
        to: Some(convert_account(accounts[1])),
        value: U256::from(10000),
        gas_price: U256::from(1000000000),
        gas: U256::from(21000),
        data: Vec::new()
    };

    let signed_tx = tx.sign(&get_private_key());

    let tx_hash = web3.eth().send_raw_transaction(Bytes::from(signed_tx)).wait().unwrap();

    let balance_after = web3.eth().balance(accounts[1], None).wait().unwrap();

    println!("TX Hash: {:?}", tx_hash);
    println!("Balance before: {}", balance_before);
    println!("Balance after: {}", balance_after);
}

fn get_private_key() -> H256 {
    let private_key = hex::decode(
        "4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d").unwrap();

    return H256(to_array(private_key.as_slice()));
}

fn convert_u256(value: web3::types::U256) -> U256 {
    let web3::types::U256(ref arr) = value;
    let mut ret = [0; 4];
    ret[0] = arr[0];
    ret[1] = arr[1];
    U256(ret)
}

fn convert_account(value: web3::types::H160) -> H160 {
    let ret = H160::from(value.0);
    ret
}

fn to_array(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}

