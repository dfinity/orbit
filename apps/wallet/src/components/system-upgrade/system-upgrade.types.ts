import { SystemUpgradeTarget, WasmModuleExtraChunks } from '~/generated/station/station.did';

export interface SystemUpgradeFormValue {
  target?: SystemUpgradeTarget;
  wasmModule?: ArrayBuffer;
  wasmInitArg?: string;
  wasmModuleExtraChunks?: WasmModuleExtraChunks;
  comment?: string;
}

export enum SystemUpgradeFormMode {
  Registry = 'registry',
  Advanced = 'advanced',
}

export enum SystemUpgradeScreen {
  Form = 'form',
  Confirm = 'confirm',
}
