import { describe, expect, it } from 'vitest';
import { REQUEST_DIALOG_QUERY_PARAM, STATION_ID_QUERY_PARAM } from '~/core/constants.core';
import { BlockchainStandard, BlockchainType } from '~/types/chain.types';
import {
  getRequestUrl,
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

describe('Request Url generation', () => {
  it('Generates the URL from the request ID, station ID and origin', () => {
    const urlString = getRequestUrl('abcd', '123', 'https://example.com');

    const url = new URL(urlString);

    expect(url.searchParams.get(STATION_ID_QUERY_PARAM)).toEqual('123');
    expect(url.searchParams.get(REQUEST_DIALOG_QUERY_PARAM)).toEqual('abcd');
    expect(url.origin).toEqual('https://example.com');
  });
});
