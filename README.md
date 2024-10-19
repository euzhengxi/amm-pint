# Automated Market Maker (AMM) Contract

This project is an implementation of an Automated Market Maker (AMM) smart contract. The contract supports providing liquidity, removing liquidity, token swaps, staking liquidity provider (LP) tokens, and claiming rewards based on staking duration.

## Contract Features

- **Provide Liquidity**: Users can provide two tokens, Token A and Token B, in a pool to earn LP tokens representing their share of the liquidity.
- **Remove Liquidity**: Users can remove liquidity from the pool by redeeming their LP tokens for Token A and Token B.
- **Swap Tokens**: Users can swap between Token A and Token B. A fee is charged on swaps, which is distributed to liquidity providers.
- **Staking Liquidity**: Liquidity providers can stake their LP tokens to earn rewards over time.
- **Claiming Rewards**: Users who stake their LP tokens can claim rewards based on how long their tokens were staked.

## Contract Components

### Storage Variables

- **token_a_balance**: The balance of Token A in the liquidity pool.
- **token_b_balance**: The balance of Token B in the liquidity pool.
- **total_liquidity**: The total amount of liquidity in the pool.
- **lp_balances**: The mapping of LP token balances for each user.
- **fee_rate**: The fee rate applied during swaps (e.g., 30 for 0.3%).
- **accumulated_fees**: The total fees accumulated from swaps to be distributed to liquidity providers.
- **staked_balances**: The amount of LP tokens staked by each user.
- **stake_start_time**: The timestamp when a user starts staking.
- **rewards_pool**: The pool of rewards available for distribution to stakers.
- **reward_rate**: The rate at which rewards are distributed for staking.

### Contract Functions

- **ProvideLiquidity**: Allows a user to provide liquidity in the form of Token A and Token B.
- **RemoveLiquidity**: Allows a user to remove liquidity from the pool and redeem LP tokens for Token A and Token B.
- **SwapTokens**: Facilitates token swaps between Token A and Token B, applying a fee which is distributed to liquidity providers.
- **StakeLiquidity**: Allows LP token holders to stake their tokens and start earning rewards.
- **ClaimRewards**: Allows users to claim their rewards based on the staking duration.

### Authorization

The contract supports two types of authorization:
- **Signed (Secp256k1Signature)**: A signature-based authorization for users.
- **Predicate (PredicateAddress)**: An authorization mechanism using predicates.

