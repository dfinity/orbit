import { Actor, ActorSubclass, HttpAgent } from '@icp-sdk/core/agent';
import { Principal } from '@icp-sdk/core/principal';
import { idlFactory } from '~/generated/upgrader';
import { _SERVICE, GetDisasterRecoveryStateResponse } from '~/generated/upgrader/upgrader.did';
import { CustomIdlAgent } from '~/utils/agent.utils';
import * as helperUtils from '~/utils/helper.utils';

export class UpgraderService {
  // This is the default actor that can perform all calls, including query calls.
  private actor: ActorSubclass<_SERVICE>;

  constructor(
    private agent: HttpAgent,
    private upgraderId: Principal = Principal.anonymous(),
    private idl: string,
  ) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent: this.agent,
      canisterId: this.upgraderId,
    });
  }

  async getDisasterRecoveryState(): Promise<GetDisasterRecoveryStateResponse> {
    const response = await this.actor.get_disaster_recovery_state();

    if (helperUtils.variantIs(response, 'Err')) {
      throw response.Err;
    }

    return response.Ok;
  }

  async getDisasterRecoveryStateUntyped(): Promise<string> {
    const customIdlAgent = new CustomIdlAgent({
      agent: this.agent,
      idl: this.idl,
      canisterId: this.upgraderId,
    });

    return await customIdlAgent.query('get_disaster_recovery_state', '()');
  }

  async submitRecoveryUntyped(args: ArrayBuffer) {
    const customIdlAgent = new CustomIdlAgent({
      agent: this.agent,
      idl: this.idl,
      canisterId: this.upgraderId,
    });

    return await customIdlAgent.update('request_disaster_recovery', args);
  }

  async getLogs() {
    const response = await this.actor.get_logs({ pagination: [] });

    if (helperUtils.variantIs(response, 'Err')) {
      throw response.Err;
    }

    return response.Ok;
  }

  async getLogsUntyped(): Promise<string> {
    const customIdlAgent = new CustomIdlAgent({
      agent: this.agent,
      idl: this.idl,
      canisterId: this.upgraderId,
    });

    return await customIdlAgent.query('get_logs', '(record { })');
  }
}
