export enum BlockchainType {
  InternetComputer = 'icp',
  Bitcoin = 'btc',
  Ethereum = 'eth',
}

export enum BlockchainStandard {
  Native = 'native',
}

export interface FetchTransfersInput {
  from_dt?: Date;
  limit?: number;
}

export interface WalletIncomingTransfer {
  from: string;
  amount: bigint;
  fee: bigint;
  created_at?: Date;
  confirmations?: number;
}

export interface FetchTransfersResponse {
  transfers: WalletIncomingTransfer[];
}

export interface WalletApi {
  fetchBalance(): Promise<bigint>;

  fetchTransfers(input: FetchTransfersInput): Promise<WalletIncomingTransfer[]>;
}
