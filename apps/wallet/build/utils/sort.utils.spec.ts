import { describe, expect, it } from 'vitest';
import { compareSemanticVersions } from './sort.utils';

describe('Build sort utils', () => {
  describe('compareSemanticVersions', () => {
    it.each([
      { unsorted: ['0.0.1', '1.0.0'], sorted: ['1.0.0', '0.0.1'] },
      { unsorted: ['0.0.1-alpha.1', '0.0.2'], sorted: ['0.0.2', '0.0.1-alpha.1'] },
      {
        unsorted: ['0.0.1-alpha.3', '0.0.1-alpha.10'],
        sorted: ['0.0.1-alpha.10', '0.0.1-alpha.3'],
      },
      { unsorted: ['0.0.1-alpha.1', '0.0.1-beta.1'], sorted: ['0.0.1-beta.1', '0.0.1-alpha.1'] },
      {
        unsorted: ['0.0.1-alpha.1', '1', '0.1', '2.0.3-beta'],
        sorted: ['2.0.3-beta', '1', '0.1', '0.0.1-alpha.1'],
      },
    ])('should sort ordered by newest $unsorted into $sorted', ({ unsorted, sorted }) => {
      const result = unsorted.sort(compareSemanticVersions());

      expect(result).toEqual(sorted);
    });

    it.each([
      { unsorted: ['0.0.1', '1.0.0'], sorted: ['0.0.1', '1.0.0'] },
      { unsorted: ['0.0.1-alpha.1', '0.0.2'], sorted: ['0.0.1-alpha.1', '0.0.2'] },
      {
        unsorted: ['0.0.1-alpha.3', '0.0.1-alpha.10'],
        sorted: ['0.0.1-alpha.3', '0.0.1-alpha.10'],
      },
      { unsorted: ['0.0.1-alpha.1', '0.0.1-beta.1'], sorted: ['0.0.1-alpha.1', '0.0.1-beta.1'] },
      {
        unsorted: ['0.0.1-alpha.1', '1', '0.1', '2.0.3-beta'],
        sorted: ['0.0.1-alpha.1', '0.1', '1', '2.0.3-beta'],
      },
    ])('should sort ordered by oldest $unsorted into $sorted', ({ unsorted, sorted }) => {
      const result = unsorted.sort(compareSemanticVersions('oldest'));

      expect(result).toEqual(sorted);
    });
  });
});
