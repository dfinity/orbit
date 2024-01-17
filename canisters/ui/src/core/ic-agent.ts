import { AnonymousIdentity, HttpAgent } from '@dfinity/agent';
import { appInitConfig } from '~/configs';
import { logger } from '~/core';
import { loadIdentity } from '~/workers/utils';

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
    const identity = (await loadIdentity()) ?? new AnonymousIdentity();

    await this.init();

    icAgent.get().replaceIdentity(identity);
    if (identity.getPrincipal().isAnonymous()) {
      logger.warn('Using anonymous identity, make sure to sign in');
    }
  }
}

const icAgent = new IcAgent();

export { icAgent };
