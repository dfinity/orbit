//! dfx-orbit wallet management commands.

use candid::Principal;
use clap::{Parser, Subcommand};

/// Wallet management commands.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum OrbitWalletArgs {
    Add(AddWallet),
    List(ListWallets),
    Rename(RenameWallet),
    Remove(RemoveWallet),
}

/// Adds an Orbit wallet.
#[derive(Debug, Parser)]
pub struct AddWallet {
    /// Wallet name.
    #[structopt(long)]
    name: String,
    /// Wallet canister ID.
    #[structopt(long)]
    canister_id: Principal,
}

/// Lists Orbit wallets.
#[derive(Debug, Parser)]
pub struct ListWallets {
    /// List all wallets.
    #[structopt(long)]
    all: bool,
}

/// Renames an Orbit wallet.
#[derive(Debug, Parser)]
pub struct RenameWallet {
    /// Wallet name.
    #[structopt(long)]
    name: String,
    /// New wallet name.
    #[structopt(long)]
    new_name: String,
}

/// Removes an Orbit wallet.
#[derive(Debug, Parser)]
pub struct RemoveWallet {
    /// Wallet name.
    #[structopt(long)]
    name: String,
}
