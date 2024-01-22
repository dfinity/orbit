import { Identity } from '@dfinity/agent';
import { AuthClient, IdbStorage, KEY_STORAGE_DELEGATION } from '@dfinity/auth-client';
import { DelegationChain } from '@dfinity/identity';
import { appInitConfig } from '~/configs';

export class AuthService {
  // 1 hour in nanoseconds
  static readonly maxAuthTTL = BigInt(60 * 60 * 1000 * 1000 * 1000);

  constructor(private authClient?: AuthClient) {}

  invalidateAuthClient(): void {
    this.authClient = undefined;
  }

  async client(): Promise<AuthClient> {
    if (!this.authClient) {
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
        })
        .catch(reject);
    });
  }

  async logout(): Promise<void> {
    const client = await this.client();

    await client.logout();

    this.invalidateAuthClient();
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
