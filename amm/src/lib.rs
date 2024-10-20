// lib.rs
use anyhow::bail;
use essential_types::{Key, Value, Word};

/// Module containing the AMM contract ABI.
#[allow(missing_docs)]
pub mod amm {
    pint_abi::gen_from_file! {
        abi: "../pint/out/debug/amm-abi.json",
        contract:  "../pint/out/debug/amm.json",
    }
}

pub mod claim_rewards;
pub mod provide_liquidity;
pub mod remove_liquidity;
pub mod stake_liquidity;
pub mod swap_tokens;

/// Represents a query result, which may or may not contain a value.
pub struct Query(pub Option<Value>);

/// Generates the key for querying an account's balance.
pub fn balance_key(hashed_key: [Word; 4]) -> Key {
    let balance: Vec<_> = amm::storage::keys::keys()
        .lp_balances(|e| e.entry(hashed_key))
        .into();
    balance.into_iter().next().expect("Must be a key")
}

/// Generates the key for querying an account's LP balance.
pub fn lp_balance_key(hashed_key: [Word; 4]) -> Key {
    let key: Vec<_> = amm::storage::keys::keys()
        .lp_balances(|e| e.entry(hashed_key))
        .into();
    key.into_iter().next().expect("Must be a key")
}

/// Generates the key for querying an account's nonce.
pub fn nonce_key(hashed_key: [Word; 4]) -> Key {
    let keys: Vec<_> = amm::storage::keys::keys()
        .nonce(|e| e.entry(hashed_key))
        .into();
    keys.into_iter().next().expect("Must be a key")
}

/// Extracts the nonce from a Query result.
pub fn nonce(nonce: Query) -> anyhow::Result<Word> {
    let r = match nonce.0 {
        Some(nonce) => match &nonce[..] {
            [] => 0,
            [nonce] => *nonce,
            _ => bail!("Expected single word, got: {:?}", nonce),
        },
        None => 0,
    };
    Ok(r)
}

/// Extracts the balance from a Query result.
pub fn balance(balance: Query) -> anyhow::Result<Word> {
    let r = match balance.0 {
        Some(balance) => match &balance[..] {
            [] => 0,
            [balance] => *balance,
            _ => bail!("Expected single word, got: {:?}", balance),
        },
        None => 0,
    };
    Ok(r)
}
