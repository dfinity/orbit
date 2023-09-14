import { Identity } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';

export class AuthService {
  constructor(private authClient?: AuthClient) {}

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
          onSuccess: () => resolve(client.getIdentity()),
        })
        .catch(reject);
    });
  }

  async logout(): Promise<void> {
    const client = await this.client();

    await client.logout();
  }
}
