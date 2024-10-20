// main.rs
use clap::{Parser, Subcommand};
use essential_app_utils::compile::compile_pint_project;
use essential_rest_client::{builder_client::EssentialBuilderClient, node_client::EssentialNodeClient};
use essential_wallet::Wallet;
use std::path::PathBuf;
use amm::{Query, ProvideLiquidityArgs, RemoveLiquidityArgs, SwapTokensArgs, StakeLiquidityArgs, ClaimRewardsArgs};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    wallet: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
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
    let wallet = {
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
            let ca = provide_liquidity(wallet, args).await?;
            println!("Sent provide liquidity solution: {}", ca);
        }
        Command::RemoveLiquidity(args) => {
            println!(
                "Removing liquidity: user={}, lp_tokens={}",
                args.user, args.lp_tokens
            );
            let ca = remove_liquidity(wallet, args).await?;
            println!("Sent remove liquidity solution: {}", ca);
        }
        Command::SwapTokens(args) => {
            println!(
                "Swapping tokens: user={}, from_token={}, amount_in={}",
                args.user, args.from_token, args.amount_in
            );
            let ca = swap_tokens(wallet, args).await?;
            println!("Sent swap tokens solution: {}", ca);
        }
        Command::StakeLiquidity(args) => {
            println!(
                "Staking liquidity: user={}, amount={}, current_time={}",
                args.user, args.amount, args.current_time
            );
            let ca = stake_liquidity(wallet, args).await?;
            println!("Sent stake liquidity solution: {}", ca);
        }
        Command::ClaimRewards(args) => {
            println!(
                "Claiming rewards: user={}, current_time={}",
                args.user, args.current_time
            );
            let ca = claim_rewards(wallet, args).await?;
            println!("Sent claim rewards solution: {}", ca);
        }
    }

    Ok(())
}

async fn provide_liquidity(wallet: Wallet, args: ProvideLiquidityArgs) -> anyhow::Result<String> {
    // Logic for providing liquidity
    let address = compile_address(args.pint_directory).await?;
    let hashed_key = hash_key(&wallet, &args.user);
    let node = EssentialNodeClient::new(args.node_api)?;
    let builder = EssentialBuilderClient::new(args.builder_api)?;

    let init = amm::provide_liquidity::Init {
        hashed_key,
        amount_a: args.amount_a,
        amount_b: args.amount_b,
    };

    let to_sign = amm::provide_liquidity::data_to_sign(init)?;
    let sig = wallet.sign_words(&to_sign.to_words(), &args.user)?;

    let build_solution = amm::provide_liquidity::BuildSolution {
        hashed_key,
        amount_a: to_sign.amount_a,
        amount_b: to_sign.amount_b,
        signature: sig,
    };

    let solution = amm::provide_liquidity::build_solution(build_solution)?;
    let ca = builder.submit_solution(&solution).await?;
    Ok(ca.to_string())
}

async fn remove_liquidity(wallet: Wallet, args: RemoveLiquidityArgs) -> anyhow::Result<String> {
    // Similar implementation as above for removing liquidity
    Ok("remove_liquidity_result".to_string())
}

async fn swap_tokens(wallet: Wallet, args: SwapTokensArgs) -> anyhow::Result<String> {
    // Similar implementation for swapping tokens
    Ok("swap_tokens_result".to_string())
}

async fn stake_liquidity(wallet: Wallet, args: StakeLiquidityArgs) -> anyhow::Result<String> {
    // Similar implementation for staking liquidity
    Ok("stake_liquidity_result".to_string())
}

async fn claim_rewards(wallet: Wallet, args: ClaimRewardsArgs) -> anyhow::Result<String> {
    // Similar implementation for claiming rewards
    Ok("claim_rewards_result".to_string())
}

fn hash_key(wallet: &Wallet, account_name: &str) -> [Word; 4] {
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
