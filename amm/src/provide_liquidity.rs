// provide_liquidity.rs
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{solution::{Solution, SolutionData}, Word};

use crate::{Query, lp_balance_key};

pub struct Init {
    pub hashed_key: [Word; 4],
    pub amount_a: Word,
    pub amount_b: Word,
}

pub struct ToSign {
    pub hashed_key: [Word; 4],
    pub amount_a: Word,
    pub amount_b: Word,
}

pub struct BuildSolution {
    pub hashed_key: [Word; 4],
    pub amount_a: Word,
    pub amount_b: Word,
    pub signature: RecoverableSignature,
}

impl ToSign {
    /// Converts the ToSign struct to a vector of Words for signing.
    pub fn to_words(&self) -> Vec<Word> {
        vec![
            self.hashed_key[0],
            self.hashed_key[1],
            self.hashed_key[2],
            self.hashed_key[3],
            self.amount_a,
            self.amount_b,
        ]
    }
}

pub fn data_to_sign(init: Init) -> anyhow::Result<ToSign> {
    Ok(ToSign {
        hashed_key: init.hashed_key,
        amount_a: init.amount_a,
        amount_b: init.amount_b,
    })
}

pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        hashed_key,
        amount_a,
        amount_b,
        signature,
    } = build;

    let pub_vars = crate::amm::ProvideLiquidity::PubVars {
        user: hashed_key,
        amount_a,
        amount_b,
    };

    let signature = signature.encode();
    let vars = crate::amm::ProvideLiquidity::Vars {
        auth: crate::amm::LiquidityAuth::Signed(signature),
    };

    let mutations = crate::amm::storage::mutations()
        .lp_balances(|map| map.entry(hashed_key, amount_a + amount_b))
        .token_a_balance(amount_a)
        .token_b_balance(amount_b)
        .total_liquidity(amount_a - amount_b);

    let solution = SolutionData {
        predicate_to_solve: crate::amm::ProvideLiquidity::ADDRESS,
        decision_variables: vars.into(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };

    Ok(Solution {
        data: vec![solution],
    })
}
