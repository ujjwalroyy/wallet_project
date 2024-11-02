use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;

#[derive(CandidType, Deserialize)]
struct Wallet {
    balance: u64,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet { balance: 0 }
    }
    
    pub fn get_balance(&self) -> u64 {
        self.balance
    }
    
    pub fn send_tokens(&mut self, amount: u64, recipient: &mut Wallet) -> Result<(), String> {
        if self.balance < amount {
            return Err("Insufficient balance".to_string());
        }
        self.balance -= amount;
        recipient.balance += amount;
        Ok(())
    }
    
    pub fn receive_tokens(&mut self, amount: u64) {
        self.balance += amount;
    }
}

#[update]
fn initialize_wallet() -> Wallet {
    Wallet::new()
}

#[update]
fn send_tokens(wallet: &mut Wallet, amount: u64, recipient: &mut Wallet) -> Result<(), String> {
    wallet.send_tokens(amount, recipient)
}

#[query]
fn get_balance(wallet: &Wallet) -> u64 {
    wallet.get_balance()
}
