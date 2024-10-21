export enum BlockchainType {
  InternetComputer = 'icp',
  Bitcoin = 'btc',
  Ethereum = 'eth',
}

export enum BlockchainStandard {
  Native = 'icp_native',
  ICRC1 = 'icrc1',
}

export enum AddressFormat {
  ICPNative = 'icp_account_identifier',
  ICRC1 = 'icrc1_account',
}

export enum TokenSymbol {
  ICP = 'ICP',
}

export interface FetchTransfersInput {
  fromDt?: Date;
  toDt?: Date;
}

export interface AccountIncomingTransfer {
  from: string;
  to: string;
  amount: bigint;
  fee: bigint;
  created_at?: Date;
  confirmations?: number;
}

export interface FetchTransfersResponse {
  transfers: AccountIncomingTransfer[];
}

export enum ChainApiCapability {
  Balance,
  Transfers,
}

export interface ChainApi {
  fetchBalance(): Promise<bigint>;

  fetchTransfers(input: FetchTransfersInput): Promise<AccountIncomingTransfer[]>;

  isValidAddress(address: string): boolean;

  getCapabilities(): ChainApiCapability[];
}
