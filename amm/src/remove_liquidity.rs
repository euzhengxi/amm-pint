// remove_liquidity.rs
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{solution::{Solution, SolutionData}, Word};

use crate::{Query, lp_balance_key};

pub struct Init {
    pub hashed_key: [Word; 4],
    pub lp_tokens: Word,
}

pub struct ToSign {
    pub hashed_key: [Word; 4],
    pub lp_tokens: Word,
}

pub struct BuildSolution {
    pub hashed_key: [Word; 4],
    pub lp_tokens: Word,
    pub signature: RecoverableSignature,
}

pub fn data_to_sign(init: Init) -> anyhow::Result<ToSign> {
    Ok(ToSign {
        hashed_key: init.hashed_key,
        lp_tokens: init.lp_tokens,
    })
}

pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        hashed_key,
        lp_tokens,
        signature,
    } = build;

    let pub_vars = crate::amm::RemoveLiquidity::PubVars {
        user: hashed_key,
        lp_tokens,
    };

    let signature = signature.encode();
    let vars = crate::amm::RemoveLiquidity::Vars {
        auth: crate::amm::LiquidityAuth::Signed(signature),
    };

    let mutations = crate::amm::storage::mutations()
        .lp_balances(|map| map.entry(hashed_key, lp_tokens))
        .token_a_balance(lp_tokens)
        .token_b_balance(lp_tokens)
        .total_liquidity(lp_tokens);

    let solution = SolutionData {
        predicate_to_solve: crate::amm::RemoveLiquidity::ADDRESS,
        decision_variables: vars.into(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };

    Ok(Solution {
        data: vec![solution],
    })
}
