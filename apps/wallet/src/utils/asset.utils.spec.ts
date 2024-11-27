import { shortenIcrc1Address } from './asset.utils';
import { describe, expect, it } from 'vitest';

describe('shortenIcrc1Address', () => {
  it('returns the principal if the subaccount is not present', () => {
    expect(shortenIcrc1Address('rwlgt-iiaaa-aaaaa-aaaaa-cai')).toBe('rwlgt-iiaaa-aaaaa-aaaaa-cai');
    expect(
      shortenIcrc1Address('wmzac-nabae-aqcai-baeaq-caiba-eaqca-ibaea-qcaib-aeaqc-aibae-aqc'),
    ).toBe('wmzac-nabae-...-aibae-aqc');
  });

  it('returns some of the principal and some of the subaccount if the subaccount is present', () => {
    expect(
      shortenIcrc1Address(
        'wmzac-nabae-aqcai-baeaq-caiba-eaqca-ibaea-qcaib-aeaqc-aibae-aqc-haltvua.102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f',
      ),
    ).toBe('wmzac-...102030405060708090a0...1e1f');

    expect(
      shortenIcrc1Address(
        'rwlgt-iiaaa-aaaaa-aaaaa-cai-pyz4egi.102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f',
      ),
    ).toBe('rwlgt-...102030405060708090a0...1e1f');

    expect(shortenIcrc1Address('rwlgt-iiaaa-aaaaa-aaaaa-cai-ltrlami.10203')).toBe('rwlgt-...10203');
  });
});
