export const isSetAndNotFalse = (value: unknown) => {
  if (value === 'false' || value === false || value === undefined || value === null) {
    return false;
  }

  return true;
};
