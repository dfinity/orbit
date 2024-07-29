import { ChangeCanisterTarget } from '~/generated/station/station.did';

export interface ChangeCanisterFormValue {
  target?: ChangeCanisterTarget;
  wasmModule?: ArrayBuffer;
  wasmInitArg?: string;
  comment?: string;
}

export enum ChangeCanisterFormMode {
  Registry = 'registry',
  Advanced = 'advanced',
}

export enum ChangeCanisterScreen {
  Form = 'form',
  Confirm = 'confirm',
}
