import { AnonymousIdentity, HttpAgent } from '@icp-sdk/core/agent';
import { appInitConfig } from '~/configs/init.config';
import { logger } from './logger.core';
import { AuthService } from '~/services/auth.service';

class IcAgent {
  private isReady = false;

  constructor(
    private agent: HttpAgent = new HttpAgent({ host: appInitConfig.apiGatewayUrl.toString() }),
  ) {
    this.agent = agent;
  }

  async init(): Promise<void> {
    if (this.isReady) {
      return;
    }

    if (!appInitConfig.isProduction) {
      await this.agent.fetchRootKey();
    }

    this.isReady = true;
  }

  get(): HttpAgent {
    return this.agent;
  }

  async loadIdentity(): Promise<void> {
    const authService = new AuthService();
    const identity = (await authService.identity()) ?? new AnonymousIdentity();

    await this.init();

    icAgent.get().replaceIdentity(identity);
    if (identity.getPrincipal().isAnonymous()) {
      logger.warn('Using anonymous identity, make sure to sign in');
    }
  }
}

const icAgent = new IcAgent();

export { icAgent };
