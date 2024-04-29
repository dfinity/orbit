import { Resource } from '~/generated/station/station.did';
import { AggregatedResoucePermissions } from '~/types/permissions.types';

export const useResourcesFromAggregatedView = (
  aggregatedView: AggregatedResoucePermissions[],
): Resource[] => {
  const resources: Resource[] = [];

  for (const view of aggregatedView) {
    resources.push(...view.resources.map(r => r.resource));
  }

  return resources;
};
