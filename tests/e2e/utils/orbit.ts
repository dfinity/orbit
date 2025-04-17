import { spawnSync } from 'child_process';

export function publishArtifact(artifact: string) {
  const result = spawnSync('orbit-cli', ['registry', 'publish', '--app', artifact]);

  if (result.status !== 0) {
    throw new Error(`Failed to publish artifact: ${result.stderr.toString()}`);
  }
}

export function copyArtifact(artifact: string) {
  const result = spawnSync('cp', ['wasms/' + artifact + '.wasm.gz', 'artifacts/' + artifact]);

  if (result.status !== 0) {
    throw new Error(`Failed to copy artifact: ${result.stderr.toString()}`);
  }
}

export function topUpAccount(icpAccountIdentifier: string, amount: number) {
  const result = spawnSync('dfx', [
    'ledger',
    'transfer',
    icpAccountIdentifier,
    '--memo',
    '0',
    '--icp',
    amount.toString(),
  ]);

  if (result.status !== 0) {
    throw new Error(`Failed to top up account: ${result.stderr.toString()}`);
  }
}
