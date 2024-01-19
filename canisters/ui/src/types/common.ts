export type Maybe<T> = T | null;

export type ExtractOk<T> = T extends { Ok: infer U } ? U : never;
