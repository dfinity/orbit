import { Certificate, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { icAgent } from '~/core/ic-agent.core';
import { isSemanticVersion } from '~/utils/helper.utils';
import logger from '~/core/logger.core';
import { appInitConfig } from '~/configs/init.config';
import { redirectToKey } from '~/plugins/router.plugin';

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

function redirectToURL(redirectTo: URL): void {
  const url = new URL(redirectTo.href);
  const pathParts = url.pathname.split('/').filter(Boolean);
  const requestedVersion = isSemanticVersion(pathParts?.[0] ?? '', 'v') ? pathParts[0] : undefined;

  if (!requestedVersion) {
    window.location.href = redirectTo.href;

    return;
  }

  // Since the asset canister does not support nested fallbacks for index.html yet, we need to redirect
  // to the exact path of the requested version so that it's index.html can be served correctly by the asset canister.
  //
  // The hook after the user logs in will redirect the user back to the requested path based on the session storage.
  const unversionedUrl = new URL(window.location.href);
  unversionedUrl.pathname = '/' + pathParts.slice(1).join('/');
  window.sessionStorage.setItem(redirectToKey, unversionedUrl.href);

  const exactPathUrl = new URL(window.location.href);
  exactPathUrl.pathname = '/' + requestedVersion + '/';

  window.location.href = exactPathUrl.href;
}

/**
 * Fetch the compatibility file for the UI, if a version is provided, then fetch the versioned file.
 */
async function fetchCompatFile(version?: string): Promise<typeof window.__compat__> {
  const fileUrl = version
    ? `${appInitConfig.baseUrl}${version}/compat.json`
    : `${appInitConfig.baseUrl}compat.json`;

  const response = await fetch(fileUrl);
  return await response.json();
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
      const url = new URL(window.location.href);
      const pathParts = url.pathname.split('/').filter(Boolean); // Remove empty strings
      const requestedVersion = isSemanticVersion(pathParts?.[0] ?? '', 'v')
        ? pathParts[0]
        : undefined;

      const [stationApiVersion, compat, rootCompat] = requestedVersion
        ? await Promise.all([
            fetchStationApiVersion(agent, stationId),
            fetchCompatFile(requestedVersion),
            // Also fetch the root compatibility file to get the latest supported version of the UI.
            fetchCompatFile(),
          ])
        : await Promise.all([fetchStationApiVersion(agent, stationId), window.__compat__, null]);

      // If the requested version is the latest version supported by the UI, then the user gets redirected to
      // the unversioned path.
      if (requestedVersion && rootCompat?.api.latest === stationApiVersion) {
        const parts = pathParts.filter(part => !isSemanticVersion(part, 'v'));
        url.pathname = '/' + parts.join('/');

        if (opts.redirectIfIncompatible) {
          redirectToURL(url);
        }

        return url;
      }

      // If the latest version of the API supported by the ui is the same as the
      // station API version, then it is compatible.
      if (compat.api.latest === stationApiVersion) {
        return;
      }

      const compatibility = compat.api.compatibility;

      // If the station API version is newer than the latest supported version, then we treat it as incompatible
      // and redirect to the unversioned path to avoid breaking the UI. It will also get redirected
      // if the compatibility file does not have the station API version.
      if (
        stationApiVersion > compat.api.latest ||
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
          redirectToURL(url);
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
        redirectToURL(url);
      }

      return url;
    },
  };
};
