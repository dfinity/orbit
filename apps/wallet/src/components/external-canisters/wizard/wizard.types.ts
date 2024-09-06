import { Principal } from '@dfinity/principal';
import { Allow, RequestPolicyRule, UUID } from '~/generated/station/station.did';
import { ExternalCanisterStateEnum } from '~/types/station.types';

export enum CanisterWizardSetupStep {
  Configuration = 1,
  Permission = 2,
  ApprovalPolicy = 3,
}

export interface ApprovalPolicyModel {
  policy_id?: UUID;
  rule?: RequestPolicyRule;
}

export interface CanisterWizardModel {
  configuration: CanisterConfigurationModel;
  permission: CanisterPermissionModel;
  approvalPolicy: CanisterApprovalPolicyModel;
}

export interface CanisterConfigurationModel {
  id?: UUID;
  canisterId?: Principal;
  name?: string;
  description?: string;
  labels?: string[];
  state?: ExternalCanisterStateEnum;
  createdAt?: string;
  modifiedAt?: string;
  maybe_with_initial_cycles?: bigint;
}

export interface CanisterPermissionModel {
  read: Allow;
  change: Allow;
}

export interface CanisterApprovalPolicyModel {
  change: ApprovalPolicyModel[];
}
