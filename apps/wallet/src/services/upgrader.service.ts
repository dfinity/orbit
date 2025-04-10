import { Actor, ActorSubclass, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { idlFactory } from '~/generated/upgrader';
import { _SERVICE, GetDisasterRecoveryStateResponse } from '~/generated/upgrader/upgrader.did';
import * as helperUtils from '~/utils/helper.utils';

export class UpgraderService {
  // This actor is modified to only perform calls that can be verified, such as update calls that go through consensus.
  private verified_actor: ActorSubclass<_SERVICE>;

  // This is the default actor that can perform all calls, including query calls.
  private actor: ActorSubclass<_SERVICE>;

  constructor(
    private agent: HttpAgent,
    private upgraderId: Principal = Principal.anonymous(),
  ) {
    this.actor = Actor.createActor<_SERVICE>(idlFactory, {
      agent: this.agent,
      canisterId: this.upgraderId,
    });

    this.verified_actor = Actor.createActor<_SERVICE>(
      helperUtils.transformIdlWithOnlyVerifiedCalls(idlFactory),
      {
        agent: this.agent,
        canisterId: this.upgraderId,
      },
    );
  }

  async getDisasterRecoveryState(): Promise<GetDisasterRecoveryStateResponse> {
    const response = await this.actor.get_disaster_recovery_state();

    if (helperUtils.variantIs(response, 'Err')) {
      throw response.Err;
    }

    return response.Ok;
  }

  async submitRecovery(args: ArrayBuffer) {
    try {
      const response = await this.agent.call(this.upgraderId.toText(), {
        methodName: 'request_disaster_recovery',
        arg: args,
      });

      // this.agent.

      console.log(response);
    } catch (e) {
      console.error(e);
    }
  }
}
