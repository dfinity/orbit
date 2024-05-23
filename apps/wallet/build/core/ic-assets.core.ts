import { writeFileSync } from 'fs';
import { resolve } from 'path';
import { PRODUCTION } from './configs.core';

export const getContentSecurityPolicy = (): string => {
  const csp: Record<string, string[]> = {
    'default-src': ["'none'"],
    'script-src': ["'self'", "'unsafe-eval'"],
    'connect-src': ["'self'", 'https://icp-api.io', 'https://ic0.app', 'https://icp0.io'],
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

  if (!PRODUCTION) {
    csp['connect-src'].push('localhost:4943');
  }

  return Object.entries(csp)
    .map(([key, value]) => {
      return `${key} ${value.join(' ')}`;
    })
    .join('; ');
};

export const generateICAssetsJson = (assetsDir = 'public', fileName = '.ic-assets.json') => {
  const assetsJsonDir = resolve(__dirname, '../..', assetsDir, fileName);
  const assetsConfig = {
    well_known: {
      match: '.well-known',
      ignore: false,
    },
    all: {
      match: '**/*',
      headers: {
        'X-Frame-Options': 'DENY',
        'X-Content-Type-Options': 'nosniff',
        'Referrer-Policy': 'same-origin',
        'Content-Security-Policy': getContentSecurityPolicy(),
        'Strict-Transport-Security': 'max-age=31536000; includeSubDomains',
        'X-XSS-Protection': '1; mode=block',
      },
      allow_raw_access: false,
    },
    fonts: {
      match: '**/fonts/**/*',
      headers: {
        'Cache-Control': 'max-age=31536000',
      },
    },
    assets: {
      match: '**/assets/**/*',
      headers: {
        'Cache-Control': 'max-age=604800',
      },
    },
    images: {
      match: '**/images/**/*',
      headers: {
        'Cache-Control': 'max-age=604800',
      },
    },
  };

  const icAssetsJson = Object.values(assetsConfig);
  writeFileSync(assetsJsonDir, JSON.stringify(icAssetsJson, null, 2), {
    encoding: 'utf-8',
  });
};
