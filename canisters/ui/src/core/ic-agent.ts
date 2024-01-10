import { HttpAgent } from '@dfinity/agent';
import { appInitConfig } from '~/configs';

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

  get() {
    return this.agent;
  }
}

const icAgent = new IcAgent();

export { icAgent };
