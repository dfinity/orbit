// Hand-written TypeScript subset of the station API. Only the types the audit
// command needs are modelled here; everything else is opaque. Keeping the
// surface small avoids a `didc bind` dependency for the first audit release.
//
// Cross-reference: core/station/api/spec.did.

export type UUID = string;

export type UserStatus = { Active: null } | { Inactive: null };

export interface UserGroup {
  id: UUID;
  name: string;
}

export interface User {
  id: UUID;
  name: string;
  status: UserStatus;
  groups: UserGroup[];
}

export type UserSpecifier = { Any: null } | { Id: UUID[] } | { Group: UUID[] };

export interface Quorum {
  approvers: UserSpecifier;
  min_approved: number;
}

export interface QuorumPercentage {
  approvers: UserSpecifier;
  min_approved: number;
}

export type RequestPolicyRule =
  | { AutoApproved: null }
  | { Quorum: Quorum }
  | { QuorumPercentage: QuorumPercentage }
  | { AllowListed: null }
  | { AllowListedByMetadata: unknown }
  | { AnyOf: RequestPolicyRule[] }
  | { AllOf: RequestPolicyRule[] }
  | { Not: RequestPolicyRule }
  | { NamedRule: UUID };

export type ResourceId = { Any: null } | { Id: UUID };
export type ResourceIds = { Any: null } | { Ids: UUID[] };

export interface CanisterMethod {
  canister_id: string;
  method_name: string;
}

export type ValidationMethodResourceTarget = { No: null } | { ValidationMethod: CanisterMethod };

export type ExecutionMethodResourceTarget = { Any: null } | { ExecutionMethod: CanisterMethod };

export interface CallExternalCanisterResourceTarget {
  validation_method: ValidationMethodResourceTarget;
  execution_method: ExecutionMethodResourceTarget;
}

export type RequestSpecifier =
  | { AddAccount: null }
  | { AddUser: null }
  | { EditAccount: ResourceIds }
  | { EditUser: ResourceIds }
  | { Transfer: ResourceIds }
  | { AddAddressBookEntry: null }
  | { EditAddressBookEntry: ResourceIds }
  | { RemoveAddressBookEntry: ResourceIds }
  | { SystemUpgrade: null }
  | { SetDisasterRecovery: null }
  | { ChangeExternalCanister: unknown }
  | { FundExternalCanister: unknown }
  | { CreateExternalCanister: null }
  | { CallExternalCanister: CallExternalCanisterResourceTarget }
  | { EditPermission: unknown }
  | { AddRequestPolicy: null }
  | { EditRequestPolicy: ResourceIds }
  | { RemoveRequestPolicy: ResourceIds }
  | { AddUserGroup: null }
  | { EditUserGroup: ResourceIds }
  | { RemoveUserGroup: ResourceIds }
  | { ManageSystemInfo: null }
  | { AddAsset: null }
  | { EditAsset: ResourceIds }
  | { RemoveAsset: ResourceIds }
  | { AddNamedRule: null }
  | { EditNamedRule: ResourceIds }
  | { RemoveNamedRule: ResourceIds };

export interface RequestPolicy {
  id: UUID;
  specifier: RequestSpecifier;
  rule: RequestPolicyRule;
}

export interface AssetMetadata {
  key: string;
  value: string;
}

export interface Asset {
  id: UUID;
  blockchain: string;
  standards: string[];
  symbol: string;
  name: string;
  metadata: AssetMetadata[];
  decimals: number;
}

export interface NamedRule {
  id: UUID;
  name: string;
  description?: [string] | [];
  rule: RequestPolicyRule;
}

export type ExternalCanisterResourceAction =
  | { List: null }
  | { Create: null }
  | { Change: unknown }
  | { Read: unknown }
  | { Fund: unknown }
  | { Call: CallExternalCanisterResourceTarget };

export type Resource =
  | { Permission: unknown }
  | { Account: unknown }
  | { AddressBook: unknown }
  | { ExternalCanister: ExternalCanisterResourceAction }
  | { Notification: unknown }
  | { Request: unknown }
  | { RequestPolicy: unknown }
  | { System: unknown }
  | { User: unknown }
  | { UserGroup: unknown }
  | { Asset: unknown }
  | { NamedRule: unknown };

export interface Permission {
  resource: Resource;
  allow: unknown;
}
