import { Principal } from '@dfinity/principal';
import {
  Allow,
  CanisterInstallMode,
  CanisterMethod,
  ExternalCanisterChangeRequestPolicyRuleInput,
  LogVisibility,
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
  log_visibility?: LogVisibility;
  wasm_memory_limit?: number;
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

export interface CanisterAllowedMethod {
  methodName: string;
  validationTarget: ValidationMethodResourceTarget;
}

export interface CanisterConfiguredMethodCall extends CanisterAllowedMethod {
  methodName: string;
  validationTarget: ValidationMethodResourceTarget;
  permission?: Allow;
  requestPolicies: ExternalCanisterChangeRequestPolicyRuleInput[];
}

export interface CanisterCallModel {
  canisterId?: Principal;
  methodName?: string;
  arg?: Uint8Array;
  requestComment?: string;
  cycles?: bigint;
  validationTarget?: ValidationMethodResourceTarget;
}

export interface CanisterCallReviewContext {
  canisterId: Principal;
  methodName: string;
  arg?: Uint8Array;
  argChecksum?: string;
  argValidationRendering?: string;
  cycles?: bigint;
  validationMethod?: CanisterMethod;
  reply?: Uint8Array;
  candidIdl?: string;
}

export interface CanisterSnapshot {
  snapshotId: string;
  totalSize: number;
  takenAtTimestamp: string;
}

export interface CanisterCreateSnapshotModel {
  canisterId?: Principal;
  comment?: string;
}

export interface CanisterRestoreSnapshotModel {
  canisterId?: Principal;
  snapshotId?: string;
  comment?: string;
}

export interface CanisterRemoveSnapshotModel {
  canisterId?: Principal;
  snapshotId?: string;
  comment?: string;
}
