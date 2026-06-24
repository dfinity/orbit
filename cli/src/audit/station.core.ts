import { execAsync } from '../utils';
import { Asset, NamedRule, Permission, RequestPolicy, User, UserGroup } from './types';

export interface StationContext {
  station: string;
  network: string;
  identity: string;
}

const PAGE_SIZE = 50;

// Escapes a value for safe inclusion in a single-quoted POSIX shell argument.
// A literal `'` inside single quotes is impossible, so the standard idiom is to
// close the string, emit an escaped quote, and reopen: foo'bar -> 'foo'\''bar'.
const shq = (value: string): string => `'${value.replace(/'/g, "'\\''")}'`;

const dfx = async (ctx: StationContext, method: string, args: string): Promise<unknown> => {
  // Candid arguments are always tuples — even single-arg calls need the outer `( ... )`.
  // `method` and `args` come from this file (not user input); `identity` / `network` /
  // `station` come from CLI flags and are shell-escaped to prevent injection.
  const cmd = `dfx canister call --identity ${shq(ctx.identity)} --network ${shq(ctx.network)} --output json ${shq(ctx.station)} ${method} '(${args})'`;
  const raw = await execAsync(cmd);
  return JSON.parse(raw);
};

const unwrapOk = <T>(response: unknown, method: string): T => {
  if (response && typeof response === 'object' && 'Ok' in response) {
    return (response as { Ok: T }).Ok;
  }
  throw new Error(`Station call '${method}' failed: ${JSON.stringify(response)}`);
};

const readOffset = (next_offset: unknown): number => {
  if (Array.isArray(next_offset) && next_offset.length > 0) {
    return Number(next_offset[0]);
  }
  return 0;
};

export const listRequestPolicies = async (ctx: StationContext): Promise<RequestPolicy[]> => {
  const policies: RequestPolicy[] = [];
  let offset = 0;
  do {
    const args = `record { offset = opt ${offset}; limit = opt ${PAGE_SIZE}; }`;
    const response = await dfx(ctx, 'list_request_policies', args);
    const page = unwrapOk<{ policies: RequestPolicy[]; next_offset?: unknown }>(
      response,
      'list_request_policies',
    );
    policies.push(...page.policies);
    offset = readOffset(page.next_offset);
  } while (offset > 0);
  return policies;
};

export const listUsers = async (ctx: StationContext): Promise<User[]> => {
  const users: User[] = [];
  let offset = 0;
  do {
    const args = `record { paginate = opt record { offset = opt ${offset}; limit = opt ${PAGE_SIZE}; }; statuses = null; groups = null; search_term = null; }`;
    const response = await dfx(ctx, 'list_users', args);
    const page = unwrapOk<{ users: User[]; next_offset?: unknown }>(response, 'list_users');
    users.push(...page.users);
    offset = readOffset(page.next_offset);
  } while (offset > 0);
  return users;
};

export const listUserGroups = async (ctx: StationContext): Promise<UserGroup[]> => {
  const groups: UserGroup[] = [];
  let offset = 0;
  do {
    const args = `record { paginate = opt record { offset = opt ${offset}; limit = opt ${PAGE_SIZE}; }; search_term = null; }`;
    const response = await dfx(ctx, 'list_user_groups', args);
    const page = unwrapOk<{ user_groups: UserGroup[]; next_offset?: unknown }>(
      response,
      'list_user_groups',
    );
    groups.push(...page.user_groups);
    offset = readOffset(page.next_offset);
  } while (offset > 0);
  return groups;
};

export const listAssets = async (ctx: StationContext): Promise<Asset[]> => {
  const assets: Asset[] = [];
  let offset = 0;
  do {
    const args = `record { paginate = opt record { offset = opt ${offset}; limit = opt ${PAGE_SIZE}; }; }`;
    const response = await dfx(ctx, 'list_assets', args);
    const page = unwrapOk<{ assets: Asset[]; next_offset?: unknown }>(response, 'list_assets');
    assets.push(...page.assets);
    offset = readOffset(page.next_offset);
  } while (offset > 0);
  return assets;
};

export const listPermissions = async (ctx: StationContext): Promise<Permission[]> => {
  const permissions: Permission[] = [];
  let offset = 0;
  do {
    const args = `record { resources = null; paginate = opt record { offset = opt ${offset}; limit = opt ${PAGE_SIZE}; }; }`;
    const response = await dfx(ctx, 'list_permissions', args);
    const page = unwrapOk<{ permissions: Permission[]; next_offset?: unknown }>(
      response,
      'list_permissions',
    );
    permissions.push(...page.permissions);
    offset = readOffset(page.next_offset);
  } while (offset > 0);
  return permissions;
};

export const listNamedRules = async (ctx: StationContext): Promise<NamedRule[]> => {
  const rules: NamedRule[] = [];
  let offset = 0;
  do {
    const args = `record { paginate = opt record { offset = opt ${offset}; limit = opt ${PAGE_SIZE}; }; }`;
    const response = await dfx(ctx, 'list_named_rules', args);
    const page = unwrapOk<{ named_rules: NamedRule[]; next_offset?: unknown }>(
      response,
      'list_named_rules',
    );
    rules.push(...page.named_rules);
    offset = readOffset(page.next_offset);
  } while (offset > 0);
  return rules;
};
