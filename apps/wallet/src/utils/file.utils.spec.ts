import { describe, expect, it } from 'vitest';
import { readFileAsArrayBuffer } from './file.utils';
import { flushPromises } from '@vue/test-utils';
import { mockFileReader } from '~/test.utils';

describe('readFileAsArrayBuffer', () => {
  it('successfully reads a file and returns an ArrayBuffer', async () => {
    global.FileReader = mockFileReader(new ArrayBuffer(10));

    const mockFile = new File(['dummy content'], 'test.txt', { type: 'text/plain' });
    const arrayBuffer = await readFileAsArrayBuffer(mockFile);

    await flushPromises();

    expect(arrayBuffer).toBeInstanceOf(ArrayBuffer);
    expect(arrayBuffer.byteLength).toBe(10);
  });

  it('handles FileReader errors', async () => {
    global.FileReader = mockFileReader(new ArrayBuffer(10), true);

    const mockFile = new File(['dummy content'], 'test.txt', { type: 'text/plain' });

    await expect(readFileAsArrayBuffer(mockFile)).rejects.toBeTruthy();
  });
});
