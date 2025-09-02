import { Identity } from '@icp-sdk/core/agent';
import { AuthClient, IdbStorage, KEY_STORAGE_DELEGATION } from '@icp-sdk/core/auth-client';
import { DelegationChain } from '@icp-sdk/core/identity';
import { appInitConfig } from '~/configs/init.config';

export class AuthService {
  // 8 hours in nanoseconds
  static readonly maxAuthTTL = BigInt(8 * 60 * 60 * 1000 * 1000 * 1000);

  constructor(private authClient?: AuthClient) {}

  async client(args: { reset?: boolean } = { reset: false }): Promise<AuthClient> {
    if (!this.authClient || args.reset) {
      this.authClient = await AuthClient.create({
        idleOptions: {
          disableIdle: true,
          disableDefaultIdleCallback: true,
        },
      });
    }

    return this.authClient;
  }

  async identity(): Promise<Identity | null> {
    const client = await this.client();
    const isAuthenticated = await client.isAuthenticated();

    return isAuthenticated ? client.getIdentity() : null;
  }

  async login(): Promise<Identity> {
    const client = await this.client();

    return new Promise((resolve, reject) => {
      client
        .login({
          maxTimeToLive: AuthService.maxAuthTTL,
          onSuccess: () => resolve(client.getIdentity()),
          onError: reject,
          identityProvider: appInitConfig.providers.internetIdentity,
          derivationOrigin: appInitConfig.derivationOrigin,
        })
        .catch(reject);
    });
  }

  async logout(): Promise<void> {
    const client = await this.client();

    await client.logout();

    await this.client({ reset: true });
  }

  async getRemainingSessionTimeMs(): Promise<number | null> {
    const idbStorage: IdbStorage = new IdbStorage();
    const delegationChain: string | null = await idbStorage.get(KEY_STORAGE_DELEGATION);
    const maybeDelegation =
      delegationChain !== null ? DelegationChain.fromJSON(delegationChain) : null;

    if (maybeDelegation) {
      const maybeExpirationTime: bigint | undefined =
        maybeDelegation.delegations[0]?.delegation.expiration;

      if (maybeExpirationTime) {
        const remainingTimeMs = Number(maybeExpirationTime / 1_000_000n) - Date.now();

        return remainingTimeMs;
      }
    }

    return null;
  }
}
