// swap_tokens.rs
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{solution::{Solution, SolutionData}, Word};

use crate::{Query, lp_balance_key};

pub struct Init {
    pub hashed_key: [Word; 4],
    pub from_token: Word,
    pub amount_in: Word,
}

pub struct ToSign {
    pub hashed_key: [Word; 4],
    pub from_token: Word,
    pub amount_in: Word,
}

pub struct BuildSolution {
    pub hashed_key: [Word; 4],
    pub from_token: Word,
    pub amount_in: Word,
    pub signature: RecoverableSignature,
}

pub fn data_to_sign(init: Init) -> anyhow::Result<ToSign> {
    Ok(ToSign {
        hashed_key: init.hashed_key,
        from_token: init.from_token,
        amount_in: init.amount_in,
    })
}

pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        hashed_key,
        from_token,
        amount_in,
        signature,
    } = build;

    let pub_vars = crate::amm::SwapTokens::PubVars {
        user: hashed_key,
        from_token,
        amount_in,
    };

    let signature = signature.encode();
    let vars = crate::amm::SwapTokens::Vars {
        auth: crate::amm::LiquidityAuth::Signed(signature),
    };

    let mutations = crate::amm::storage::mutations()
        .token_a_balance(if from_token == 1 { amount_in } else { -amount_in })
        .token_b_balance(if from_token == 2 { amount_in } else { -amount_in });

    let solution = SolutionData {
        predicate_to_solve: crate::amm::SwapTokens::ADDRESS,
        decision_variables: vars.into(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };

    Ok(Solution {
        data: vec![solution],
    })
}
