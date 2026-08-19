import { ActorSubclass } from '@dfinity/agent';
import { createStationActor } from './agent';
import { IdentitySource, loadIdentity } from './identity';
import { Asset, NamedRule, Permission, RequestPolicy, User, UserGroup } from './types';

export interface StationContext {
  station: string;
  network: string;
  identity: string;
  identitySource: IdentitySource;
}

const PAGE_SIZE = 50n;

interface OkOf<T> {
  Ok: T;
}

const unwrapOk = <T>(response: unknown, method: string): T => {
  if (response && typeof response === 'object' && 'Ok' in response) {
    return (response as OkOf<T>).Ok;
  }
  throw new Error(`Station call '${method}' failed: ${JSON.stringify(response)}`);
};

const nextOffset = (next_offset: Array<bigint> | []): bigint | null =>
  next_offset.length > 0 ? next_offset[0] : null;

const paginated = async <Item>(
  method: string,
  call: (pageArg: { offset: bigint; limit: bigint }) => Promise<unknown>,
  extract: (page: unknown) => { items: Item[]; next: Array<bigint> | [] },
): Promise<Item[]> => {
  const items: Item[] = [];
  let offset: bigint | null = 0n;
  while (offset !== null) {
    const response = await call({ offset, limit: PAGE_SIZE });
    const ok = unwrapOk<unknown>(response, method);
    const page = extract(ok);
    items.push(...page.items);
    offset = nextOffset(page.next);
  }
  return items;
};

/**
 * Builds an authenticated agent-js actor for the target station.
 *
 * Reused across every `list_*` call within a single audit run so we don't
 * pay agent + root-key bootstrap cost per query.
 */
export const buildStationActor = async (ctx: StationContext): Promise<ActorSubclass> => {
  const identity = loadIdentity(ctx.identitySource, ctx.identity);
  return createStationActor(ctx.station, ctx.network, identity);
};

export const listRequestPolicies = (actor: ActorSubclass): Promise<RequestPolicy[]> =>
  paginated<RequestPolicy>(
    'list_request_policies',
    p => actor.list_request_policies({ offset: [p.offset], limit: [p.limit] }) as Promise<unknown>,
    page => {
      const ok = page as { policies: RequestPolicy[]; next_offset: Array<bigint> | [] };
      return { items: ok.policies, next: ok.next_offset };
    },
  );

export const listUsers = (actor: ActorSubclass): Promise<User[]> =>
  paginated<User>(
    'list_users',
    p =>
      actor.list_users({
        paginate: [{ offset: [p.offset], limit: [p.limit] }],
        statuses: [],
        groups: [],
        search_term: [],
      }) as Promise<unknown>,
    page => {
      const ok = page as { users: User[]; next_offset: Array<bigint> | [] };
      return { items: ok.users, next: ok.next_offset };
    },
  );

export const listUserGroups = (actor: ActorSubclass): Promise<UserGroup[]> =>
  paginated<UserGroup>(
    'list_user_groups',
    p =>
      actor.list_user_groups({
        paginate: [{ offset: [p.offset], limit: [p.limit] }],
        search_term: [],
      }) as Promise<unknown>,
    page => {
      const ok = page as { user_groups: UserGroup[]; next_offset: Array<bigint> | [] };
      return { items: ok.user_groups, next: ok.next_offset };
    },
  );

export const listAssets = (actor: ActorSubclass): Promise<Asset[]> =>
  paginated<Asset>(
    'list_assets',
    p =>
      actor.list_assets({
        paginate: [{ offset: [p.offset], limit: [p.limit] }],
      }) as Promise<unknown>,
    page => {
      const ok = page as { assets: Asset[]; next_offset: Array<bigint> | [] };
      return { items: ok.assets, next: ok.next_offset };
    },
  );

export const listPermissions = (actor: ActorSubclass): Promise<Permission[]> =>
  paginated<Permission>(
    'list_permissions',
    p =>
      actor.list_permissions({
        resources: [],
        paginate: [{ offset: [p.offset], limit: [p.limit] }],
      }) as Promise<unknown>,
    page => {
      const ok = page as { permissions: Permission[]; next_offset: Array<bigint> | [] };
      return { items: ok.permissions, next: ok.next_offset };
    },
  );

export const listNamedRules = (actor: ActorSubclass): Promise<NamedRule[]> =>
  paginated<NamedRule>(
    'list_named_rules',
    p =>
      actor.list_named_rules({
        paginate: [{ offset: [p.offset], limit: [p.limit] }],
      }) as Promise<unknown>,
    page => {
      const ok = page as { named_rules: NamedRule[]; next_offset: Array<bigint> | [] };
      return { items: ok.named_rules, next: ok.next_offset };
    },
  );
