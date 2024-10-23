// claim_rewards.rs
use essential_app_utils::inputs::Encode;
use essential_sign::secp256k1::ecdsa::RecoverableSignature;
use essential_types::{solution::{Solution, SolutionData}, Word};

pub struct Init {
    pub hashed_key: [Word; 4],
    pub current_time: Word,
}

pub struct ToSign {
    pub hashed_key: [Word; 4],
    pub current_time: Word,
}

pub struct BuildSolution {
    pub hashed_key: [Word; 4],
    pub current_time: Word,
    pub signature: RecoverableSignature,
}

pub fn data_to_sign(init: Init) -> anyhow::Result<ToSign> {
    Ok(ToSign {
        hashed_key: init.hashed_key,
        current_time: init.current_time,
    })
}

pub fn build_solution(build: BuildSolution) -> anyhow::Result<Solution> {
    let BuildSolution {
        hashed_key,
        current_time,
        signature,
    } = build;

    let _pub_vars = crate::amm::ClaimRewards::PubVars {
        user: hashed_key,
        current_time,
    };

    let _signature = signature.encode();

    let mutations = crate::amm::storage::mutations()
        .rewards_pool(-10) // Replace with actual reward calculation
        .token_a_balance(10); // Replace with actual reward calculation

    let solution = SolutionData {
        predicate_to_solve: crate::amm::ClaimRewards::ADDRESS,
        decision_variables: Default::default(),
        transient_data: Default::default(),
        state_mutations: mutations.into(),
    };

    Ok(Solution {
        data: vec![solution],
    })
}
