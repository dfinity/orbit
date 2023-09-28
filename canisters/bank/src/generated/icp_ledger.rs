// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

pub type SubAccount = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize)]
pub struct Account { owner: Principal, subaccount: Option<SubAccount> }

#[derive(CandidType, Deserialize)]
pub struct FeatureFlags { icrc2: bool }

#[derive(CandidType, Deserialize)]
pub struct UpgradeArgs {
  maximum_number_of_accounts: Option<u64>,
  icrc1_minting_account: Option<Account>,
  feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize)]
pub struct Tokens { e8s: u64 }

pub type TextAccountIdentifier = String;
#[derive(CandidType, Deserialize)]
pub struct Duration { secs: u64, nanos: u32 }

#[derive(CandidType, Deserialize)]
pub struct ArchiveOptions {
  num_blocks_to_archive: u64,
  trigger_threshold: u64,
  max_message_size_bytes: Option<u64>,
  cycles_for_archive_creation: Option<u64>,
  node_max_memory_size_bytes: Option<u64>,
  controller_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
  send_whitelist: Vec<Principal>,
  token_symbol: Option<String>,
  transfer_fee: Option<Tokens>,
  minting_account: TextAccountIdentifier,
  maximum_number_of_accounts: Option<u64>,
  accounts_overflow_trim_quantity: Option<u64>,
  transaction_window: Option<Duration>,
  max_message_size_bytes: Option<u64>,
  icrc1_minting_account: Option<Account>,
  archive_options: Option<ArchiveOptions>,
  initial_values: Vec<(TextAccountIdentifier,Tokens,)>,
  token_name: Option<String>,
  feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize)]
pub enum LedgerCanisterPayload { Upgrade(Option<UpgradeArgs>), Init(InitArgs) }

pub type AccountIdentifier = serde_bytes::ByteBuf;
#[derive(CandidType, Deserialize)]
pub struct AccountBalanceArgs { account: AccountIdentifier }

#[derive(CandidType, Deserialize)]
pub struct AccountBalanceArgsDfx { account: TextAccountIdentifier }

#[derive(CandidType, Deserialize)]
pub struct Archive { canister_id: Principal }

#[derive(CandidType, Deserialize)]
pub struct Archives { archives: Vec<Archive> }

#[derive(CandidType, Deserialize)]
pub struct DecimalsRet { decimals: u32 }

pub type Icrc1Tokens = candid::Nat;
#[derive(CandidType, Deserialize)]
pub enum Value {
  Int(candid::Int),
  Nat(candid::Nat),
  Blob(serde_bytes::ByteBuf),
  Text(String),
}

#[derive(CandidType, Deserialize)]
pub struct Icrc1SupportedStandardsRetItem { url: String, name: String }

pub type Icrc1Timestamp = u64;
#[derive(CandidType, Deserialize)]
pub struct TransferArg {
  to: Account,
  fee: Option<Icrc1Tokens>,
  memo: Option<serde_bytes::ByteBuf>,
  from_subaccount: Option<SubAccount>,
  created_at_time: Option<Icrc1Timestamp>,
  amount: Icrc1Tokens,
}

pub type Icrc1BlockIndex = candid::Nat;
#[derive(CandidType, Deserialize)]
pub enum Icrc1TransferError {
  GenericError{ message: String, error_code: candid::Nat },
  TemporarilyUnavailable,
  BadBurn{ min_burn_amount: Icrc1Tokens },
  Duplicate{ duplicate_of: Icrc1BlockIndex },
  BadFee{ expected_fee: Icrc1Tokens },
  CreatedInFuture{ ledger_time: u64 },
  TooOld,
  InsufficientFunds{ balance: Icrc1Tokens },
}

#[derive(CandidType, Deserialize)]
pub enum Icrc1TransferResult { Ok(Icrc1BlockIndex), Err(Icrc1TransferError) }

#[derive(CandidType, Deserialize)]
pub struct AllowanceArgs { account: Account, spender: Account }

#[derive(CandidType, Deserialize)]
pub struct TimeStamp { timestamp_nanos: u64 }

#[derive(CandidType, Deserialize)]
pub struct Allowance { allowance: Icrc1Tokens, expires_at: Option<TimeStamp> }

#[derive(CandidType, Deserialize)]
pub struct ApproveArgs {
  fee: Option<Icrc1Tokens>,
  memo: Option<serde_bytes::ByteBuf>,
  from_subaccount: Option<SubAccount>,
  created_at_time: Option<TimeStamp>,
  amount: Icrc1Tokens,
  expected_allowance: Option<Icrc1Tokens>,
  expires_at: Option<TimeStamp>,
  spender: Account,
}

#[derive(CandidType, Deserialize)]
pub enum ApproveError {
  GenericError{ message: String, error_code: candid::Nat },
  TemporarilyUnavailable,
  Duplicate{ duplicate_of: Icrc1BlockIndex },
  BadFee{ expected_fee: Icrc1Tokens },
  AllowanceChanged{ current_allowance: Icrc1Tokens },
  CreatedInFuture{ ledger_time: u64 },
  TooOld,
  Expired{ ledger_time: u64 },
  InsufficientFunds{ balance: Icrc1Tokens },
}

#[derive(CandidType, Deserialize)]
pub enum ApproveResult { Ok(Icrc1BlockIndex), Err(ApproveError) }

#[derive(CandidType, Deserialize)]
pub struct NameRet { name: String }

pub type BlockIndex = u64;
#[derive(CandidType, Deserialize)]
pub struct GetBlocksArgs { start: BlockIndex, length: u64 }

pub type Memo = u64;
#[derive(CandidType, Deserialize)]
pub enum Operation {
  Approve{
    fee: Tokens,
    from: AccountIdentifier,
    allowance_e8s: candid::Int,
    allowance: Tokens,
    expires_at: Option<TimeStamp>,
    spender: AccountIdentifier,
  },
  Burn{
    from: AccountIdentifier,
    amount: Tokens,
    spender: Option<AccountIdentifier>,
  },
  Mint{ to: AccountIdentifier, amount: Tokens },
  Transfer{
    to: AccountIdentifier,
    fee: Tokens,
    from: AccountIdentifier,
    amount: Tokens,
  },
  TransferFrom{
    to: AccountIdentifier,
    fee: Tokens,
    from: AccountIdentifier,
    amount: Tokens,
    spender: AccountIdentifier,
  },
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
  memo: Memo,
  icrc1_memo: Option<serde_bytes::ByteBuf>,
  operation: Option<Operation>,
  created_at_time: TimeStamp,
}

#[derive(CandidType, Deserialize)]
pub struct Block {
  transaction: Transaction,
  timestamp: TimeStamp,
  parent_hash: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct BlockRange { blocks: Vec<Block> }

#[derive(CandidType, Deserialize)]
pub enum QueryArchiveError {
  BadFirstBlockIndex{
    requested_index: BlockIndex,
    first_valid_index: BlockIndex,
  },
  Other{ error_message: String, error_code: u64 },
}

#[derive(CandidType, Deserialize)]
pub enum QueryArchiveResult { Ok(BlockRange), Err(QueryArchiveError) }

candid::define_function!(pub QueryArchiveFn : (GetBlocksArgs) -> (
    QueryArchiveResult,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedBlocksRange {
  callback: QueryArchiveFn,
  start: BlockIndex,
  length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct QueryBlocksResponse {
  certificate: Option<serde_bytes::ByteBuf>,
  blocks: Vec<Block>,
  chain_length: u64,
  first_block_index: BlockIndex,
  archived_blocks: Vec<ArchivedBlocksRange>,
}

#[derive(CandidType, Deserialize)]
pub enum ArchivedEncodedBlocksRangeCallbackRet {
  Ok(Vec<serde_bytes::ByteBuf>),
  Err(QueryArchiveError),
}

candid::define_function!(pub ArchivedEncodedBlocksRangeCallback : (
    GetBlocksArgs,
  ) -> (ArchivedEncodedBlocksRangeCallbackRet) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedEncodedBlocksRange {
  callback: ArchivedEncodedBlocksRangeCallback,
  start: u64,
  length: u64,
}

#[derive(CandidType, Deserialize)]
pub struct QueryEncodedBlocksResponse {
  certificate: Option<serde_bytes::ByteBuf>,
  blocks: Vec<serde_bytes::ByteBuf>,
  chain_length: u64,
  first_block_index: u64,
  archived_blocks: Vec<ArchivedEncodedBlocksRange>,
}

#[derive(CandidType, Deserialize)]
pub struct SendArgs {
  to: TextAccountIdentifier,
  fee: Tokens,
  memo: Memo,
  from_subaccount: Option<SubAccount>,
  created_at_time: Option<TimeStamp>,
  amount: Tokens,
}

#[derive(CandidType, Deserialize)]
pub struct SymbolRet { symbol: String }

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
  to: AccountIdentifier,
  fee: Tokens,
  memo: Memo,
  from_subaccount: Option<SubAccount>,
  created_at_time: Option<TimeStamp>,
  amount: Tokens,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
  TxTooOld{ allowed_window_nanos: u64 },
  BadFee{ expected_fee: Tokens },
  TxDuplicate{ duplicate_of: BlockIndex },
  TxCreatedInFuture,
  InsufficientFunds{ balance: Tokens },
}

#[derive(CandidType, Deserialize)]
pub enum TransferResult { Ok(BlockIndex), Err(TransferError) }

#[derive(CandidType, Deserialize)]
pub struct TransferFeeArg {}

#[derive(CandidType, Deserialize)]
pub struct TransferFee { transfer_fee: Tokens }

pub struct Service(pub Principal);
impl Service {
  pub async fn account_balance(&self, arg0: AccountBalanceArgs) -> Result<
    (Tokens,)
  > { ic_cdk::call(self.0, "account_balance", (arg0,)).await }
  pub async fn account_balance_dfx(
    &self,
    arg0: AccountBalanceArgsDfx,
  ) -> Result<(Tokens,)> {
    ic_cdk::call(self.0, "account_balance_dfx", (arg0,)).await
  }
  pub async fn archives(&self) -> Result<(Archives,)> {
    ic_cdk::call(self.0, "archives", ()).await
  }
  pub async fn decimals(&self) -> Result<(DecimalsRet,)> {
    ic_cdk::call(self.0, "decimals", ()).await
  }
  pub async fn icrc_1_balance_of(&self, arg0: Account) -> Result<
    (Icrc1Tokens,)
  > { ic_cdk::call(self.0, "icrc1_balance_of", (arg0,)).await }
  pub async fn icrc_1_decimals(&self) -> Result<(u8,)> {
    ic_cdk::call(self.0, "icrc1_decimals", ()).await
  }
  pub async fn icrc_1_fee(&self) -> Result<(Icrc1Tokens,)> {
    ic_cdk::call(self.0, "icrc1_fee", ()).await
  }
  pub async fn icrc_1_metadata(&self) -> Result<(Vec<(String,Value,)>,)> {
    ic_cdk::call(self.0, "icrc1_metadata", ()).await
  }
  pub async fn icrc_1_minting_account(&self) -> Result<(Option<Account>,)> {
    ic_cdk::call(self.0, "icrc1_minting_account", ()).await
  }
  pub async fn icrc_1_name(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "icrc1_name", ()).await
  }
  pub async fn icrc_1_supported_standards(&self) -> Result<
    (Vec<Icrc1SupportedStandardsRetItem>,)
  > { ic_cdk::call(self.0, "icrc1_supported_standards", ()).await }
  pub async fn icrc_1_symbol(&self) -> Result<(String,)> {
    ic_cdk::call(self.0, "icrc1_symbol", ()).await
  }
  pub async fn icrc_1_total_supply(&self) -> Result<(Icrc1Tokens,)> {
    ic_cdk::call(self.0, "icrc1_total_supply", ()).await
  }
  pub async fn icrc_1_transfer(&self, arg0: TransferArg) -> Result<
    (Icrc1TransferResult,)
  > { ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await }
  pub async fn icrc_2_allowance(&self, arg0: AllowanceArgs) -> Result<
    (Allowance,)
  > { ic_cdk::call(self.0, "icrc2_allowance", (arg0,)).await }
  pub async fn icrc_2_approve(&self, arg0: ApproveArgs) -> Result<
    (ApproveResult,)
  > { ic_cdk::call(self.0, "icrc2_approve", (arg0,)).await }
  pub async fn name(&self) -> Result<(NameRet,)> {
    ic_cdk::call(self.0, "name", ()).await
  }
  pub async fn query_blocks(&self, arg0: GetBlocksArgs) -> Result<
    (QueryBlocksResponse,)
  > { ic_cdk::call(self.0, "query_blocks", (arg0,)).await }
  pub async fn query_encoded_blocks(&self, arg0: GetBlocksArgs) -> Result<
    (QueryEncodedBlocksResponse,)
  > { ic_cdk::call(self.0, "query_encoded_blocks", (arg0,)).await }
  pub async fn send_dfx(&self, arg0: SendArgs) -> Result<(BlockIndex,)> {
    ic_cdk::call(self.0, "send_dfx", (arg0,)).await
  }
  pub async fn symbol(&self) -> Result<(SymbolRet,)> {
    ic_cdk::call(self.0, "symbol", ()).await
  }
  pub async fn transfer(&self, arg0: TransferArgs) -> Result<
    (TransferResult,)
  > { ic_cdk::call(self.0, "transfer", (arg0,)).await }
  pub async fn transfer_fee(&self, arg0: TransferFeeArg) -> Result<
    (TransferFee,)
  > { ic_cdk::call(self.0, "transfer_fee", (arg0,)).await }
}

