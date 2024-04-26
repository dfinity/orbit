export class UnregisteredUserError extends Error {
  constructor(message?: string) {
    super(message);

    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export class InvalidStationError extends Error {
  constructor(message?: string) {
    super(message);

    Object.setPrototypeOf(this, new.target.prototype);
  }
}

export class DisabledBackgroundPollingError extends Error {
  constructor(message?: string) {
    super(message);

    Object.setPrototypeOf(this, new.target.prototype);
  }
}
