import { Certificate, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { icAgent } from '~/core/ic-agent.core';
import { isSemanticVersion } from '~/utils/helper.utils';
import logger from '~/core/logger.core';

/**
 * Fetch the version of the station API from the canister metadata.
 *
 * This call is verified with the state tree certificate.
 */
async function fetchStationApiVersion(agent: HttpAgent, stationId: Principal): Promise<string> {
  const encoder = new TextEncoder();
  const versionPath: ArrayBuffer[] = [
    encoder.encode('canister'),
    stationId.toUint8Array(),
    encoder.encode('metadata'),
    encoder.encode('app:version'),
  ];

  const state = await agent.readState(stationId, {
    paths: [versionPath],
  });

  const certificate = await Certificate.create({
    canisterId: stationId,
    certificate: state.certificate,
    rootKey: agent.rootKey,
  });

  const version = certificate.lookup(versionPath);

  if (!version) {
    throw new Error('Version not found');
  }

  const decoder = new TextDecoder();
  const decodedVersion = decoder.decode(version);

  if (!isSemanticVersion(decodedVersion)) {
    throw new Error(
      'Invalid version format, expected semantic version but got `${' + decodedVersion + '}`',
    );
  }

  return decodedVersion;
}

/**
 * Use the compatibility layer to check if the station API version is compatible with the UI.
 */
export const useCompatibilityLayer = (agent: HttpAgent = icAgent.get()) => {
  return {
    fetchStationApiVersion: (stationId: Principal) => fetchStationApiVersion(agent, stationId),
    checkCompatibility: async (
      stationId: Principal,
      opts: { redirectIfIncompatible?: boolean } = {},
    ): Promise<URL | undefined> => {
      const stationApiVersion = await fetchStationApiVersion(agent, stationId);

      // If the latest version of the API supported by the ui is the same as the
      // station API version, then it is compatible.
      if (window.__compat__.api.latest === stationApiVersion) {
        return;
      }

      const url = new URL(window.location.href);
      const pathParts = url.pathname.split('/').filter(Boolean); // Remove empty strings
      const compatibility = window.__compat__.api.compatibility as Record<string, { ui: string[] }>;

      // If the station API version is newer than the latest supported version, then we treat it as incompatible
      // and redirect to the unversioned path to avoid breaking the UI. It will also get redirected
      // if the compatibility file does not have the station API version.
      if (
        stationApiVersion > window.__compat__.api.latest ||
        !compatibility?.[stationApiVersion]?.ui ||
        compatibility[stationApiVersion].ui.length === 0
      ) {
        // If the path does not contain a semantic version, then we are already on the unversioned path.
        if (!pathParts.find(part => isSemanticVersion(part, 'v'))) {
          return;
        }

        const parts = pathParts.filter(part => !isSemanticVersion(part, 'v'));
        url.pathname = '/' + parts.join('/');

        if (opts.redirectIfIncompatible) {
          window.location.href = url.href;
        }

        return url;
      }

      const compatibleUI = `v${compatibility[stationApiVersion].ui[0]}`;
      // If the path already contains the compatible version, then we are already on the correct path.
      if (pathParts.includes(compatibleUI)) {
        // This initial check is not for starts with to avoid infinite redirects.
        if (pathParts[0] !== compatibleUI) {
          logger.warn(
            `The path contains the compatible version, but it is not the first part of the path: ${compatibleUI}`,
          );
        }

        return;
      }

      const parts = pathParts.filter(part => !isSemanticVersion(part, 'v'));
      parts.unshift(compatibleUI);

      url.pathname = '/' + parts.join('/');

      if (opts.redirectIfIncompatible) {
        window.location.href = url.href;
      }

      return url;
    },
  };
};
