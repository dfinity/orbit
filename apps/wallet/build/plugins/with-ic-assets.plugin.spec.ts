import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'fs';
import { tmpdir } from 'os';
import { join, resolve } from 'path';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { withIcAssetsFile } from './with-ic-assets.plugin';

type IcAssetsRule = { match: string; headers?: Record<string, string> };

const CANISTER_IDS_PATH = resolve(__dirname, '../../../../canister_ids.json');

const INDEX_HTML = '<script>window.inline = true;</script><script src="/main.js"></script>';
// sha256 of the inline script above, base64 encoded.
const INLINE_SCRIPT_HASH = "'sha256-5n2DSRDGaWy2SggN68iONM84vDSa6opmut5FktFmjLE='";

const runWriteBundle = (dir: string): void => {
  const plugin = withIcAssetsFile({ isProduction: true });
  const writeBundle = plugin.writeBundle as unknown as (options: { dir: string }) => void;

  writeBundle.call({}, { dir });
};

const readHeadersForAllAssets = (dir: string): Record<string, string> => {
  const rules: IcAssetsRule[] = JSON.parse(readFileSync(join(dir, '.ic-assets.json'), 'utf-8'));

  return rules.find(rule => rule.match === '**/*')?.headers ?? {};
};

describe('with-ic-assets plugin', () => {
  let outDir: string;

  beforeEach(() => {
    outDir = mkdtempSync(join(tmpdir(), 'orbit-wallet-dist-'));
    writeFileSync(join(outDir, 'index.html'), INDEX_HTML);
  });

  afterEach(() => {
    rmSync(outDir, { recursive: true, force: true });
  });

  it('writes .ic-assets.json into the build output', () => {
    runWriteBundle(outDir);

    expect(existsSync(join(outDir, '.ic-assets.json'))).toBe(true);
  });

  it('sets the security headers on every asset', () => {
    runWriteBundle(outDir);
    const headers = readHeadersForAllAssets(outDir);

    expect(headers['X-Frame-Options']).toBe('DENY');
    expect(headers['X-Content-Type-Options']).toBe('nosniff');
    expect(headers['Content-Security-Policy']).toContain("default-src 'none'");
    expect(headers['Content-Security-Policy']).toContain("frame-ancestors 'none'");
  });

  it('allows the inline scripts of index.html by their hash', () => {
    runWriteBundle(outDir);

    expect(readHeadersForAllAssets(outDir)['Content-Security-Policy']).toContain(
      INLINE_SCRIPT_HASH,
    );
  });

  it('allows downloads from every wasm chunk store in canister_ids.json', () => {
    runWriteBundle(outDir);
    const csp = readHeadersForAllAssets(outDir)['Content-Security-Policy'] ?? '';
    const connectSrc = csp.split('; ').find(directive => directive.startsWith('connect-src '));
    const canisterIds = JSON.parse(readFileSync(CANISTER_IDS_PATH, 'utf-8'));
    const stores = Object.values(canisterIds.wasm_chunk_store ?? {}) as string[];

    expect(stores.length).toBeGreaterThan(0);
    for (const store of stores) {
      expect(connectSrc?.split(' ')).toContain(`https://${store}.icp0.io`);
    }
  });
});
