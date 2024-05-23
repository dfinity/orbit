declare global {
  interface Window {
    __compat__: {
      // The current version of the wallet dapp.
      version: string;
      // The compatibility of the wallet with the station API.
      api: {
        // The latest version of the station API that the wallet is using.
        latest: string;
        // The compatibility versions between the wallet and the station API.
        compatibility: Record<
          string,
          {
            ui: string[];
          }
        >;
      };
    };
  }
}

// this ensures that the file is treated as a module
export {};
