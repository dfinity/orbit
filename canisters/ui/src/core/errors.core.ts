export class UnregisteredUserError extends Error {
  constructor(message?: string) {
    super(message);

    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export class InvalidWalletError extends Error {
  constructor(message?: string) {
    super(message);

    Object.setPrototypeOf(this, new.target.prototype);
  }
}
