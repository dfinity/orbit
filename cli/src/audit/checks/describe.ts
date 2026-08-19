import { RequestSpecifier, ResourceIds, UserSpecifier } from '../types';

export const describeSpecifier = (specifier: UserSpecifier): string => {
  if ('Any' in specifier) return 'UserSpecifier::Any';
  if ('Id' in specifier) return `UserSpecifier::Id(${specifier.Id.length} user(s))`;
  if ('Group' in specifier) return `UserSpecifier::Group(${specifier.Group.length} group(s))`;
  return 'UserSpecifier::?';
};

const describeResourceIds = (ids: ResourceIds): string => {
  if ('Any' in ids) return 'Any';
  return `[${ids.Ids.length} id(s)]`;
};

export const describeRequestSpecifier = (specifier: RequestSpecifier): string => {
  const [variant] = Object.keys(specifier);
  const value = (specifier as Record<string, unknown>)[variant];
  if (
    value &&
    typeof value === 'object' &&
    ('Any' in (value as object) || 'Ids' in (value as object))
  ) {
    return `${variant}(${describeResourceIds(value as ResourceIds)})`;
  }
  return variant;
};
