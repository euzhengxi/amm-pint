// main.rs
use anyhow::bail;
use clap::{Parser, Subcommand, Args};
use essential_app_utils::compile::compile_pint_project;
use essential_rest_client::{builder_client::EssentialBuilderClient, node_client::EssentialNodeClient};
use essential_types::{Word};
use essential_wallet::Wallet;
use essential_signer::Signature;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    wallet: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

/// Arguments for providing liquidity from the AMM contract.
#[derive(Args)]
pub struct ProvideLiquidityArgs {
    /// The account providing liquidity.
    pub user: String,
    /// Amount of Token A to deposit.
    pub amount_a: Word,
    /// Amount of Token B to deposit.
    pub amount_b: Word,
    /// The address of the node to connect to.
    pub node_api: String,
    /// The address of the builder to connect to.
    pub builder_api: String,
    /// The directory of the PINT AMM contract.
    pub pint_directory: PathBuf,
}

/// Arguments for removing liquidity from the AMM contract.
#[derive(Args)]
pub struct RemoveLiquidityArgs {
    /// The account removing liquidity.
    pub user: String,
    /// Amount of LP tokens to remove.
    pub lp_tokens: Word,
    /// The address of the node to connect to.
    pub node_api: String,
    /// The address of the builder to connect to.
    pub builder_api: String,
    /// The directory of the PINT AMM contract.
    pub pint_directory: PathBuf,
}

/// Arguments for swapping tokens in the AMM contract.
#[derive(Args)]
pub struct SwapTokensArgs {
    /// The account performing the swap.
    pub user: String,
    /// The token to swap from (e.g., 1 for Token A, 2 for Token B).
    pub from_token: Word,
    /// Amount of tokens to swap in.
    pub amount_in: Word,
    /// The address of the node to connect to.
    pub node_api: String,
    /// The address of the builder to connect to.
    pub builder_api: String,
    /// The directory of the PINT AMM contract.
    pub pint_directory: PathBuf,
}

/// Arguments for staking liquidity in the AMM contract.
#[derive(Args)]
pub struct StakeLiquidityArgs {
    /// The account staking liquidity.
    pub user: String,
    /// Amount of LP tokens to stake.
    pub amount: Word,
    /// Current timestamp for the staking operation.
    pub current_time: Word,
    /// The address of the node to connect to.
    pub node_api: String,
    /// The address of the builder to connect to.
    pub builder_api: String,
    /// The directory of the PINT AMM contract.
    pub pint_directory: PathBuf,
}

/// Arguments for claiming rewards from the AMM contract.
#[derive(Args)]
pub struct ClaimRewardsArgs {
    /// The account claiming rewards.
    pub user: String,
    /// Current timestamp for the claim operation.
    pub current_time: Word,
    /// The address of the node to connect to.
    pub node_api: String,
    /// The address of the builder to connect to.
    pub builder_api: String,
    /// The directory of the PINT AMM contract.
    pub pint_directory: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    ProvideLiquidity(ProvideLiquidityArgs),
    RemoveLiquidity(RemoveLiquidityArgs),
    SwapTokens(SwapTokensArgs),
    StakeLiquidity(StakeLiquidityArgs),
    ClaimRewards(ClaimRewardsArgs),
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    if let Err(err) = run(args).await {
        eprintln!("Command failed because: {}", err);
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let Cli { wallet, command } = cli;
    let mut wallet = {
        let pass = rpassword::prompt_password("Enter password to unlock wallet: ")?;
        let wallet = match wallet {
            Some(path) => Wallet::new(&pass, path)?,
            None => Wallet::with_default_path(&pass)?,
        };
        wallet
    };

    match command {
        Command::ProvideLiquidity(args) => {
            println!(
                "Providing liquidity: user={}, amount_a={}, amount_b={}",
                args.user, args.amount_a, args.amount_b
            );
            let ca = provide_liquidity(&mut wallet, args).await?;
            println!("Sent provide liquidity solution: {}", ca);
        }
        Command::RemoveLiquidity(args) => {
            println!(
                "Removing liquidity: user={}, lp_tokens={}",
                args.user, args.lp_tokens
            );
            println!("Sent remove liquidity solution:", );
        }
        Command::SwapTokens(args) => {
            println!(
                "Swapping tokens: user={}, from_token={}, amount_in={}",
                args.user, args.from_token, args.amount_in
            );
            println!("Sent swap tokens solution:");
        }
        Command::StakeLiquidity(args) => {
            println!(
                "Staking liquidity: user={}, amount={}, current_time={}",
                args.user, args.amount, args.current_time
            );
            println!("Sent stake liquidity solution:");
        }
        Command::ClaimRewards(args) => {
            println!(
                "Claiming rewards: user={}, current_time={}",
                args.user, args.current_time
            );
            println!("Sent claim rewards solution:");
        }
    }

    Ok(())
}

async fn provide_liquidity(wallet: &mut Wallet, args: ProvideLiquidityArgs) -> anyhow::Result<String> {
    // Logic for providing liquidity
    let _address = compile_address(args.pint_directory).await?;
    let hashed_key = hash_key(wallet, &args.user);
    let _node = EssentialNodeClient::new(args.node_api)?;
    let builder = EssentialBuilderClient::new(args.builder_api)?;

    let init = amm_cli::provide_liquidity::Init {
        hashed_key,
        amount_a: args.amount_a,
        amount_b: args.amount_b,
    };

    let to_sign = amm_cli::provide_liquidity::data_to_sign(init)?;
    let sig = wallet.sign_words(&to_sign.to_words(), &args.user)?;
    let Signature::Secp256k1(sig) = sig else {
        bail!("Invalid signature")
    };
    let build_solution = amm_cli::provide_liquidity::BuildSolution {
        hashed_key,
        amount_a: to_sign.amount_a,
        amount_b: to_sign.amount_b,
        signature: sig,
    };

    let solution = amm_cli::provide_liquidity::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca.to_string())
}

fn hash_key(wallet: &mut Wallet, account_name: &str) -> [Word; 4] {
    let public_key = wallet.get_public_key(account_name).unwrap();
    let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
        panic!("Invalid public key")
    };
    let encoded = essential_sign::encode::public_key(&public_key);
    essential_types::convert::word_4_from_u8_32(essential_hash::hash_words(&encoded))
}

async fn compile_address(pint_directory: PathBuf) -> Result<essential_types::PredicateAddress, anyhow::Error> {
    let compiled = compile_pint_project(pint_directory).await?;
    let contract_address = essential_hash::contract_addr::from_contract(&compiled);
    let predicate_address = essential_hash::content_addr(&compiled.predicates[0]);
    let predicate_address = essential_types::PredicateAddress {
        contract: contract_address,
        predicate: predicate_address,
    };
    Ok(predicate_address)
}
