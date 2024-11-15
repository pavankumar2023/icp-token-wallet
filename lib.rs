use ic_cdk_macros::*;
use ic_cdk::storage;

#[derive(Default)]
struct Wallet {
    balance: u64,
}

#[update]
fn send_tokens(recipient: String, amount: u64) -> String {
    let wallet: &mut Wallet = storage::get_mut();
    if wallet.balance >= amount {
        wallet.balance -= amount;
        // Simulate sending to recipient
        format!("Sent {} tokens to {}", amount, recipient)
    } else {
        "Insufficient balance.".to_string()
    }
}

#[update]
fn receive_tokens(amount: u64) {
    let wallet: &mut Wallet = storage::get_mut();
    wallet.balance += amount;
}

#[query]
fn get_balance() -> u64 {
    let wallet: &Wallet = storage::get();
    wallet.balance
}
