import { HttpAgent } from '@dfinity/agent';
import { appInitConfig } from '~/configs';

class IcAgent {
  constructor(
    private agent: HttpAgent = new HttpAgent({ host: appInitConfig.apiGatewayUrl.toString() }),
  ) {
    this.agent = agent;
  }

  async init(): Promise<void> {
    if (!appInitConfig.isProduction) {
      await this.agent.fetchRootKey();
    }
  }

  get() {
    return this.agent;
  }
}

const icAgent = new IcAgent();

export { icAgent };
