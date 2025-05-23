export const walletUrl =
  process.env.WALLET_URL ?? 'http://werw6-ayaaa-aaaaa-774aa-cai.localhost:4943';

export function getWalletPath(path: string) {
  const url = new URL(walletUrl);
  url.pathname = path;
  return url.toString();
}
