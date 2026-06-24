import {
  Asset,
  CanisterMethod,
  NamedRule,
  Permission,
  RequestPolicy,
  RequestPolicyRule,
  RequestSpecifier,
  User,
  UserGroup,
} from '../types';

let counter = 0;
const nextId = (): string => `id-${++counter}`;

export const resetCounter = (): void => {
  counter = 0;
};

export const makeUser = (overrides: Partial<User> = {}): User => ({
  id: overrides.id ?? nextId(),
  name: overrides.name ?? 'Test User',
  status: overrides.status ?? { Active: null },
  groups: overrides.groups ?? [],
  ...overrides,
});

export const makeGroup = (overrides: Partial<UserGroup> = {}): UserGroup => ({
  id: overrides.id ?? nextId(),
  name: overrides.name ?? 'Test Group',
});

export const makePolicy = (
  specifier: RequestSpecifier,
  rule: RequestPolicyRule,
): RequestPolicy => ({
  id: nextId(),
  specifier,
  rule,
});

export const makeAsset = (overrides: Partial<Asset> = {}): Asset => ({
  id: overrides.id ?? nextId(),
  blockchain: overrides.blockchain ?? 'icp',
  standards: overrides.standards ?? ['icp_native'],
  symbol: overrides.symbol ?? 'ICP',
  name: overrides.name ?? 'Test Asset',
  metadata: overrides.metadata ?? [],
  decimals: overrides.decimals ?? 8,
});

export const makeNamedRule = (overrides: Partial<NamedRule>): NamedRule => ({
  id: overrides.id ?? nextId(),
  name: overrides.name ?? 'Test Named Rule',
  description: overrides.description ?? [],
  rule: overrides.rule ?? { AutoApproved: null },
});

export const makePermission = (resource: Permission['resource']): Permission => ({
  resource,
  allow: {},
});

export const method = (canister_id: string, method_name: string): CanisterMethod => ({
  canister_id,
  method_name,
});
