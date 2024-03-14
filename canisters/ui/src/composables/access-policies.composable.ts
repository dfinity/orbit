import { Resource } from '~/generated/wallet/wallet.did';
import { AggregatedResouceAccessPolicies } from '~/types/access-policies.types';

export const useResourcesFromAggregatedView = (
  aggregatedView: AggregatedResouceAccessPolicies[],
): Resource[] => {
  const resources: Resource[] = [];

  for (const view of aggregatedView) {
    resources.push(...view.resources.map(r => r.resource));
  }

  return resources;
};
