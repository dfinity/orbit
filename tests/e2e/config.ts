export const walletUrl =
  process.env.WALLET_URL ?? 'http://localhost:4943?canisterId=werw6-ayaaa-aaaaa-774aa-cai';

export function getWalletUrl(path: string) {
  const url = new URL(walletUrl);
  url.pathname = path;
  return url.toString();
}
