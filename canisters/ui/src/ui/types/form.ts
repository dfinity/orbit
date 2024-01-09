export type FormValidationRuleFn = (value: unknown) => boolean | string;
export type FormValidationRules = FormValidationRuleFn[];
export type VFormValidation = { validate: () => Promise<{ valid: boolean }>; isValid: boolean };
