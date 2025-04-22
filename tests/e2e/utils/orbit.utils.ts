import { execSync } from 'child_process';

export function publishArtifact(artifact: string) {
  console.log(`PATH: ${process.env.PATH}`);
  execSync(`orbit-cli registry publish --app ${artifact}`, { stdio: 'inherit' });
}

export function copyArtifact(artifact: string) {
  execSync(`mkdir -p artifacts/${artifact}`, { stdio: 'inherit' });
  execSync(`cp wasms/${artifact}.wasm.gz artifacts/${artifact}`, { stdio: 'inherit' });
}

export function topUpAccount(icpAccountIdentifier: string, amount: number) {
  execSync(`dfx ledger transfer ${icpAccountIdentifier} --memo 0 --icp ${amount.toString()}`, {
    stdio: 'inherit',
  });
}
