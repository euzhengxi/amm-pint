// stake_liquidity.rs
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{solution::{Solution, SolutionData}, Word};

use crate::{Query, lp_balance_key};

pub struct Init {
    pub hashed_key: [Word; 4],
    pub amount: Word,
    pub current_time: Word,
}

pub struct ToSign {
    pub hashed_key: [Word; 4],
    pub amount: Word,
    pub current_time: Word,
}

pub struct BuildSolution {
    pub hashed_key: [Word; 4],
    pub amount: Word,
    pub current_time: Word,
    pub signature: RecoverableSignature,
}

pub fn data_to_sign(init: Init) -> anyhow::Result<ToSign> {
    Ok(ToSign {
        hashed_key: init.hashed_key,
        amount: init.amount,
        current_time: init.current_time,
    })
}

pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        hashed_key,
        amount,
        current_time,
        signature,
    } = build;

    let pub_vars = crate::amm::StakeLiquidity::PubVars {
        user: hashed_key,
        amount,
        current_time,
    };

    let signature = signature.encode();
    let vars = crate::amm::StakeLiquidity::Vars {
        auth: crate::amm::LiquidityAuth::Signed(signature),
    };

    let mutations = crate::amm::storage::mutations()
        .staked_balances(|map| map.entry(hashed_key, amount))
        .stake_start_time(|map| map.entry(hashed_key, current_time));

    let solution = SolutionData {
        predicate_to_solve: crate::amm::StakeLiquidity::ADDRESS,
        decision_variables: vars.into(),
        transient_data: pub_vars.into(),
        state_mutations: mutations.into(),
    };

    Ok(Solution {
        data: vec![solution],
    })
}
