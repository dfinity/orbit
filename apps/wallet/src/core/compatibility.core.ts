import { Certificate, HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { appInitConfig } from '~/configs/init.config';
import { icAgent } from '~/core/ic-agent.core';
import logger from '~/core/logger.core';
import { redirectToKey } from '~/plugins/router.plugin';
import { isSemanticVersion } from '~/utils/helper.utils';
import { ApiCompatibilityInfo } from '~build/types/compat.types';

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
  //
  // Moreover, the asset canister behaves differently then response verification v2, for the later it
  // creates a certificate where  it will lookup if an index.html exists in nested paths recursively, which means
  // that for SPA's it would properly lookup and add the correct witness to the index.html of a path like /v1.0.0
  // even if it were to load a dynamic route like /v1.0.0/nested-route.

  // However, the asset canister if it does not have an exact path, it will always fallback to the root path for
  // the index.html which for that example it would not work for us, since it would load the
  // latest html for a versioned UI.
  const unversionedUrl = new URL(window.location.href);
  unversionedUrl.pathname = '/' + pathParts.slice(1).join('/');
  window.sessionStorage.setItem(
    redirectToKey,
    unversionedUrl.pathname + unversionedUrl.search + unversionedUrl.hash,
  );

  const exactPathUrl = new URL(window.location.href);
  exactPathUrl.pathname = '/' + requestedVersion + '/';

  window.location.href = exactPathUrl.href;
}

/**
 * Fetch the compatibility file for the UI, if a version is provided, then fetch the versioned file.
 */
async function fetchCompatFile(versionPath?: string): Promise<ApiCompatibilityInfo> {
  const fileUrl = versionPath
    ? `${appInitConfig.baseUrl}${versionPath}/compat.json`
    : `${appInitConfig.baseUrl}compat.json`;

  const response = await fetch(fileUrl);
  return await response.json();
}

/**
 * Get the currently loaded UI version from the meta tag or the import.meta.env
 */
function getCurrentlyLoadedUIVersion(): string {
  // first try to get from the index.html meta tag
  const versionMeta = document
    .querySelector('meta[name="app:build-version"]')
    ?.getAttribute('content');

  if (versionMeta && isSemanticVersion(versionMeta)) {
    return versionMeta;
  }

  return import.meta.env.APP_BUILD_VERSION;
}

/**
 * Use the compatibility layer to check if the station API version is compatible with the UI.
 *
 * Returns false if the compatible UI is not found.
 */
export const createCompatibilityLayer = (agent: HttpAgent = icAgent.get()) => {
  return {
    redirectToURL: (url: URL) => redirectToURL(url),
    fetchCompatFile: (versionPath?: string) => fetchCompatFile(versionPath),
    fetchStationApiVersion: (stationId: Principal) => fetchStationApiVersion(agent, stationId),
    async checkCompatibility(
      stationId: Principal,
      opts: { redirectIfIncompatible?: boolean } = {},
    ): Promise<URL | undefined | false> {
      const url = new URL(window.location.href);
      const pathParts = url.pathname.split('/').filter(Boolean); // Remove empty strings
      const requestedVersion = isSemanticVersion(pathParts?.[0] ?? '', 'v')
        ? pathParts[0]
        : undefined;

      const [stationApiVersion, compat, rootCompat] = requestedVersion
        ? await Promise.all([
            this.fetchStationApiVersion(stationId),
            this.fetchCompatFile(requestedVersion),
            // Also fetch the root compatibility file to get the latest supported version of the UI.
            this.fetchCompatFile(),
          ])
        : await Promise.all([this.fetchStationApiVersion(stationId), this.fetchCompatFile(), null]);

      const currentLoadedUIVersion = getCurrentlyLoadedUIVersion();

      // If the requested version is the latest version supported by the UI, then the user gets redirected to
      // the unversioned path, the same applies if the user is already in the versioned path but the ui build version
      // differs, which shows that the index.html loaded is not the correct one.
      if (
        requestedVersion &&
        (rootCompat?.api.latest === stationApiVersion ||
          currentLoadedUIVersion !== requestedVersion.slice(1))
      ) {
        const parts = pathParts.filter(part => !isSemanticVersion(part, 'v'));
        url.pathname = '/' + parts.join('/');

        if (opts.redirectIfIncompatible) {
          this.redirectToURL(url);
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
          this.redirectToURL(url);
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

      const compatibleUIExists = await this.fetchCompatFile(compatibleUI)
        .then(() => true)
        .catch(() => {
          logger.warn(`The compatible version ${compatibleUI} does not exist.`);

          return false;
        });

      if (!compatibleUIExists) {
        return false;
      }

      url.pathname = '/' + parts.join('/');

      if (opts.redirectIfIncompatible) {
        this.redirectToURL(url);
      }

      return url;
    },
  };
};
