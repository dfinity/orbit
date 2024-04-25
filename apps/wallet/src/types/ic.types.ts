export interface CanisterModule {
  wasm: Uint8Array;
  hash: {
    byteArray: Uint8Array;
    hex: string;
  };
  args?: Uint8Array;
}
