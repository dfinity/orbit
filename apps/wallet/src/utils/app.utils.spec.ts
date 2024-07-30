import { describe, expect, it } from 'vitest';
import { BlockchainStandard, BlockchainType } from '~/types/chain.types';
import {
  isCaseInsensitiveBlockchainAddress,
  maybeTransformBlockchainAddress,
} from '~/utils/app.utils';

describe('BlockchainAddress', () => {
  it('ICP Ledger account identifiers are case insensitive', () => {
    expect(
      isCaseInsensitiveBlockchainAddress(
        BlockchainType.InternetComputer,
        BlockchainStandard.Native,
      ),
    ).toBe(true);
  });

  it('ICP Ledger account identifiers should be transformed to lowercase', () => {
    const address = '0X1C916B8e8361650658d021d5bedc0ab0296ad1eb5e40c3e643f2adbd992670b5';
    expect(
      maybeTransformBlockchainAddress(
        BlockchainType.InternetComputer,
        BlockchainStandard.Native,
        address,
      ),
    ).toEqual(address.toLowerCase());
  });
});
