import { Principal } from '@dfinity/principal';
import {
  Allow,
  CanisterInstallMode,
  ExternalCanisterChangeRequestPolicyRuleInput,
  ValidationMethodResourceTarget,
} from '~/generated/station/station.did';

export interface CanisterTopUpModel {
  canisterId?: Principal;
  cycles?: bigint;
}

export interface CanisterIcSettingsModel {
  canisterId?: Principal;
  freezing_threshold?: number;
  controllers?: Principal[];
  memory_allocation?: number;
  compute_allocation?: number;
  reserved_cycles_limit?: number;
}

export interface CanisterInstallModel {
  canisterId?: Principal;
  wasmModule?: Uint8Array;
  wasmInstallArg?: Uint8Array;
  mode?: CanisterInstallMode;
}

export interface CanisterMethodCallConfigurationModel {
  canisterId: Principal;
  alreadyConfiguredMethods: CanisterConfiguredMethodCall[];
  methodName?: string;
  requestPolicies: Partial<ExternalCanisterChangeRequestPolicyRuleInput>[];
  permission: Allow;
  validationMethodName?: string;
  validationCanisterId?: Principal;
}

export interface CanisterConfiguredMethodCall {
  methodName: string;
  permission?: Allow;
  validationTarget: ValidationMethodResourceTarget;
  requestPolicies: ExternalCanisterChangeRequestPolicyRuleInput[];
}
