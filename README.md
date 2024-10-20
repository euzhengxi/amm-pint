# Automated Market Maker (AMM) Contract

This project is an implementation of an Automated Market Maker (AMM) smart contract using **PINT (Programming Intent Notation)**. The contract supports providing liquidity, removing liquidity, token swaps, staking liquidity provider (LP) tokens, and claiming rewards based on staking duration.

## Contract Features

- **Provide Liquidity**: Users can provide two tokens, Token A and Token B, to a liquidity pool and receive LP tokens representing their share.
- **Remove Liquidity**: Users can remove liquidity from the pool by redeeming their LP tokens for Token A and Token B.
- **Swap Tokens**: Users can swap between Token A and Token B. A fee is charged on swaps, which is distributed to liquidity providers.
- **Staking Liquidity**: Liquidity providers can stake their LP tokens to earn rewards over time.
- **Claiming Rewards**: Users who stake their LP tokens can claim rewards based on how long their tokens were staked.

## Contract Components

### Storage Variables

- **token_a_balance**: The balance of Token A in the liquidity pool.
- **token_b_balance**: The balance of Token B in the liquidity pool.
- **total_liquidity**: The total amount of liquidity in the pool (total LP tokens issued).
- **lp_balances**: A mapping of user addresses (`b256`) to their LP token balances.
- **fee_rate**: The fee rate applied during swaps (e.g., `30` for a 0.3% fee).
- **accumulated_fees**: The total fees accumulated from swaps to be distributed to liquidity providers.
- **staked_balances**: A mapping of user addresses to the amount of LP tokens they have staked.
- **stake_start_time**: A mapping of user addresses to the timestamp when they started staking.
- **rewards_pool**: The pool of rewards available for distribution to stakers.
- **reward_rate**: The rate at which rewards are distributed for staking.
- **nonce**: A mapping of user addresses to their nonce values, used for transaction replay protection.

### Contract Functions

- **ProvideLiquidity**: Allows a user to provide liquidity by depositing Token A and Token B into the pool. The user receives LP tokens in return.
- **RemoveLiquidity**: Allows a user to remove liquidity from the pool by redeeming their LP tokens for Token A and Token B.
- **SwapTokens**: Facilitates token swaps between Token A and Token B, applying a fee that is added to the `accumulated_fees`.
- **StakeLiquidity**: Allows LP token holders to stake their tokens to earn rewards over time.
- **ClaimRewards**: Allows users who have staked LP tokens to claim rewards based on their staking duration.

### Authorization

The contract supports two types of authorization:

- **Signed (Secp256k1Signature)**: A signature-based authorization using the Secp256k1 curve.
- **Predicate (PredicateAddress)**: An authorization mechanism using predicate addresses.

Authorization is checked in each function to ensure that only authorized users can perform actions.

### Macros

The contract includes several macros to simplify common patterns and ensure consistent behavior:

- **@auth**: A macro to handle authorization checks, verifying that the user is authorized to perform the action either by signature or predicate ownership.

  ```pint
  macro @auth($key, $auth_for, $predicate, $allowed_predicate, &rest_allow_list) {
      @is_this_predicate($auth_for);
      constraint @check_if_predicate_owns($predicate; $key) || @check_if_predicate_in_allow_list($predicate; $allowed_predicate; &rest_allow_list);
  }
  ```

- **@check_if_predicate_in_allow_list**: A recursive macro to check if a predicate is in an allowed list.
- **@check_if_predicate_owns**: Checks if the hash of the predicate address equals the user's key, ensuring ownership.
- **@is_this_predicate**: Ensures that the action is intended for the current predicate.
- **@verify_key**: Verifies a user's signature against their public key.
- **@init_once**: Initializes a state variable once, enforcing that it cannot be reinitialized.
- **@init_delta**: Initializes or updates a state variable, ensuring correct state transitions.
- **@delta**: Calculates the change in a state variable between its previous and current value.
- **@safe_increment**: Safely increments a state variable, ensuring it starts from 1 or increments by 1.

### Types

- **PredicateAddress**: A structure containing the contract address and the predicate address.
  ```pint
type PredicateAddress = { contract: b256, addr: b256 };
  ```
- **Secp256k1Signature**: A structure representing a Secp256k1 signature.
  ```pint
type Secp256k1Signature = { b256, b256, int };
  ```
- **Secp256k1PublicKey**: A structure representing a Secp256k1 public key.
  ```pint
type Secp256k1PublicKey = { b256, int };
  ```

## Usage

### Providing Liquidity
Users can provide liquidity by depositing Token A and Token B in equal proportions. The ProvideLiquidity predicate calculates the amount of LP tokens to mint for the user based on the current pool state.

**Example**:

```pint
predicate ProvideLiquidity {
    pub var user: b256 = 0xabc123...;
    pub var amount_a: int = 1000;
    pub var amount_b: int = 2000;
    var auth: LiquidityAuth = Signed(user_signature);

    // ...rest of the code...
}
```

### Removing Liquidity
Users can remove liquidity by redeeming their LP tokens. The RemoveLiquidity predicate calculates the amount of Token A and Token B the user receives based on their share of the pool.

**Example**:

```pint
predicate RemoveLiquidity {
    pub var user: b256 = 0xabc123...;
    pub var lp_tokens: int = 500;
    var auth: LiquidityAuth = Signed(user_signature);

    // ...rest of the code...
}
```

### Swapping Tokens
Users can swap between Token A and Token B using the SwapTokens predicate. A fee is applied to each swap, which is added to the accumulated_fees for distribution to liquidity providers.

**Example**:

```pint
predicate SwapTokens {
    pub var user: b256 = 0xabc123...;
    pub var from_token: int = 1; // 1 for Token A, 2 for Token B
    pub var amount_in: int = 1500;
    var auth: LiquidityAuth = Signed(user_signature);

    // ...rest of the code...
}
```

### Staking LP Tokens
Liquidity providers can stake their LP tokens using the StakeLiquidity predicate to earn rewards over time. The staking start time is recorded, and rewards accumulate based on the reward_rate.

**Example**:

```pint
predicate StakeLiquidity {
    pub var user: b256 = 0xabc123...;
    pub var amount: int = 1000;
    pub var current_time: int = 1622548800; // Unix timestamp

    // ...rest of the code...
}
```

### Claiming Rewards
Users who have staked LP tokens can claim their rewards using the ClaimRewards predicate. Rewards are calculated based on the staking duration and the reward_rate.

**Example**:

```pint
predicate ClaimRewards {
    pub var user: b256 = 0xabc123...;
    pub var current_time: int = 1625130800; // Unix timestamp

    // ...rest of the code...
}
```

## Security Considerations

- **Authorization Checks**: All predicates include authorization checks to ensure that only authorized users can perform actions.
- **Input Validation**: Constraints are used to validate input parameters (e.g., ensuring amounts are positive, balances are sufficient).
- **Replay Protection**: The nonce storage variable can be used to prevent replay attacks by ensuring each transaction is unique.
- **State Consistency**: Macros like `@init_once` and `@delta` help maintain consistent state transitions and prevent unintended modifications.

## Contributing

### Fork the Repository

Click on the 'Fork' button on the top right to create a copy of this repository on your GitHub account.

### Clone Your Fork

```bash
git clone https://github.com/yourusername/your-repo-name.git
```

### Create a New Branch

```bash
git checkout -b feature/your-feature-name
```

### Make Your Changes
Edit the code or documentation as needed.

### Commit Your Changes

```bash
git commit -am 'Add new feature'
```

### Push to Your Fork

```bash
git push origin feature/your-feature-name
```

### Submit a Pull Request
Go to the original repository and submit a pull request with a description of your changes.

## License

This project is licensed under the MIT License.

> Disclaimer: This code is for educational purposes and should be thoroughly tested and audited before use in a production environment.



