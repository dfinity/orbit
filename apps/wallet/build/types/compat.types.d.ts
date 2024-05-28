/**
 * The compatibility information of the dapp.
 *
 * This information is used to check the compatibility of the dapp with the station API.
 */
export interface ApiCompatibilityInfo {
  // The current version of the dapp.
  version: string;
  // The compatibility of the dapp with the station API.
  api: {
    // The latest version of the station API that the dapp is using.
    latest: string;
    // The compatibility versions between the dapp and the station API.
    compatibility: Record<
      string,
      {
        ui: string[];
      }
    >;
  };
}
