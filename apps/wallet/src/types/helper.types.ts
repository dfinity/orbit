export type Maybe<T> = T | null;

export type ExtractOk<T> = T extends { Ok: infer U } ? U : never;

export interface LoadableItem<T> {
  loading: boolean;
  data: T;
}

export type FormValidationRuleFn = (value: unknown) => boolean | string;
export type FormValidationRules = FormValidationRuleFn[];
export type VFormValidation = {
  validate: () => Promise<{
    valid: boolean;
    errors: Array<{ id: string; errorMessages?: string[] }>;
  }>;
  isValid: boolean;
  errors: Array<{ id: string; errorMessages?: string[] }>;
};
export type SelectItem<T = unknown> = { value: T; text: string };
