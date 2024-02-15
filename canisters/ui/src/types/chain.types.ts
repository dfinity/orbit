export enum BlockchainType {
  InternetComputer = 'icp',
  Bitcoin = 'btc',
  Ethereum = 'eth',
}

export enum BlockchainStandard {
  Native = 'native',
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

export interface ChainApi {
  fetchBalance(): Promise<bigint>;

  fetchTransfers(input: FetchTransfersInput): Promise<AccountIncomingTransfer[]>;
}
