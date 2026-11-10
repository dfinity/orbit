import { readFileSync, writeFileSync } from 'fs';
import { resolve } from 'path';
import { Plugin } from 'vite';
import { load } from 'cheerio';
import { createHash } from 'node:crypto';

// The wasm chunk stores of the control-panel registry, `wasm_chunk_store` in canister_ids.json.
// Disaster recovery downloads registry modules from their HTTP interface.
const WASM_CHUNK_STORE_ORIGINS = [
  'https://f2mu6-kiaaa-aaaam-advhq-cai.icp0.io', // production
  'https://lwcm6-3aaaa-aaaam-qb7ba-cai.icp0.io', // playground
];

const getContentSecurityPolicy = (
  isProduction: boolean,
  dynamicCspHeaders: Record<string, string[]> = {},
): string => {
  const csp: Record<string, string[]> = {
    'default-src': ["'none'"],
    'script-src': ["'self'", "'wasm-unsafe-eval'"],
    'worker-src': ["'self'", 'blob:'],
    'connect-src': [
      "'self'",
      'https://icp-api.io',
      'https://ic0.app',
      'https://icp0.io',
      'https://3r4gx-wqaaa-aaaaq-aaaia-cai.icp0.io', // SNS aggregator
      ...WASM_CHUNK_STORE_ORIGINS,
    ],
    'img-src': ["'self'", 'data:'],
    'font-src': ["'self'"],
    'object-src': ["'none'"],
    'base-uri': ["'self'"],
    'style-src': ["'self'", "'unsafe-inline'"],
    'media-src': ["'self'", 'data:', 'blob:'],
    'form-action': ["'self'"],
    'frame-ancestors': ["'none'"],
    'upgrade-insecure-requests': [],
  };

  if (!isProduction) {
    csp['connect-src'].push('localhost:4943');
  }

  return Object.entries(csp)
    .map(([key, value]) => {
      if (dynamicCspHeaders[key]) {
        value.push(...dynamicCspHeaders[key]);
      }

      return `${key} ${value.join(' ')}`;
    })
    .join('; ');
};

const createICAssetsJson = (
  isProduction: boolean,
  dynamicCspHeaders: Record<string, string[]> = {},
) => {
  return {
    well_known: {
      match: '.well-known',
      ignore: false,
    },
    well_known_ii: {
      match: '.well-known/ii-alternative-origins',
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Content-Type': 'application/json',
      },
      ignore: false,
    },
    all: {
      match: '**/*',
      headers: {
        'X-Frame-Options': 'DENY',
        'X-Content-Type-Options': 'nosniff',
        'Referrer-Policy': 'same-origin',
        'Content-Security-Policy': getContentSecurityPolicy(isProduction, dynamicCspHeaders),
        'Strict-Transport-Security': 'max-age=31536000; includeSubDomains',
        'X-XSS-Protection': '1; mode=block',
      },
      allow_raw_access: false,
    },
    assets: {
      match: '**/assets/**/*',
      headers: {
        'Cache-Control': 'max-age=604800',
      },
    },
    compatFiles: {
      match: 'compat.json',
      headers: {
        'Cache-Control': 'max-age=3600',
      },
    },
    images: {
      match: '**/images/**/*',
      headers: {
        'Cache-Control': 'max-age=604800',
      },
    },
  };
};

export const withIcAssetsFile = (
  opts: { isProduction?: boolean; fileName?: string } = {},
): Plugin => {
  const isProduction = opts.isProduction ?? true;
  const fileName = opts.fileName ?? '.ic-assets.json';

  return {
    name: 'with-ic-assets',
    writeBundle({ dir }) {
      // Written into the build output, so that the release artifact carries the headers. Release
      // deploys upload that artifact as is. Vite copies public/ before this hook runs, so a file
      // written there would only reach the output of a later build, and never a clean one.
      const icAssetsPath = resolve(dir!, fileName);
      const indexHtml = resolve(dir!, 'index.html');
      const indexContent = readFileSync(indexHtml, { encoding: 'utf-8' });
      const virtualDOM = load(indexContent);
      const dynamicCspHeaders: Record<string, string[]> = {
        'script-src': [],
      };

      // Navigate the script tags and get their sha256 hash if they are inline scripts.
      virtualDOM('script').each((_, current) => {
        const elem = virtualDOM(current);
        const src = elem.attr('src');
        if (!src) {
          const content = elem.html();
          if (content) {
            const sha256 = createHash('sha256').update(content).digest('base64');

            dynamicCspHeaders['script-src'].push(`'sha256-${sha256}'`);
          }
        }
      });

      const icAssets = createICAssetsJson(isProduction, dynamicCspHeaders);
      const icAssetsJson = Object.values(icAssets);

      writeFileSync(icAssetsPath, JSON.stringify(icAssetsJson, null, 2), {
        encoding: 'utf-8',
      });
    },
  };
};
