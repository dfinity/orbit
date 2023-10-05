export const isSetAndNotFalse = (value: unknown) => {
  if (value === 'false' || value === false || value === undefined || value === null) {
    return false;
  }

  return true;
};

// Formats a balance that is a bigint into a string with the correct number of decimals
export const formatBalance = (balance: bigint, decimals: number): string => {
  const balanceString = balance.toString();
  const balanceLength = balanceString.length;
  const balanceInteger = balanceString.slice(0, balanceLength - decimals);
  const balanceDecimal = balanceString.slice(balanceLength - decimals);

  if (balanceInteger.length === 0 && balanceDecimal === '0') {
    return '0';
  }

  return `${balanceInteger}.${balanceDecimal}`;
};
