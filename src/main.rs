mod balances;
mod proof_of_existence;
mod support;
mod system;

use crate::support::Dispatch;

/// Core type definitions for our blockchain state machine
/// These types define the fundamental data structures used throughout the system
mod types {
    /// Account identifier type - represents users in the system
    pub type AccountId = String;
    /// Balance type for handling monetary values
    pub type Balance = u128;
    /// Block number type for tracking chain progression
    pub type BlockNumber = u32;
    /// Nonce type for transaction ordering
    pub type Nonce = u32;
    /// Extrinsic type representing external transactions
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    /// Header type containing block metadata
    pub type Header = crate::support::Header<BlockNumber>;
    /// Block type combining header and transactions
    pub type Block = crate::support::Block<Header, Extrinsic>;
    /// Content type for proof of existence claims
    pub type Content = &'static str;
}

/// Main Runtime structure that orchestrates all blockchain functionality
/// Integrates different pallets (modules) and provides execution environment
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

/// Entry point for the blockchain state machine
/// Demonstrates basic blockchain operations through a series of blocks
fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer { to: charlie, amount: 20 }),
            },
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, world!",
                }),
            },
        ],
    };

    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: bob,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, world!",
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("Block 1 execution failed");
    runtime.execute_block(block_2).expect("Block 2 execution failed");
    runtime.execute_block(block_3).expect("Block 3 execution failed");

    println!("{:#?}", runtime);
}