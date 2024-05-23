import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'fs';
import { resolve } from 'path';
import { Plugin } from 'vite';
import { ENV, WALLET_VERSION } from '../core/configs.core';
import { copyFolderRecursiveSync } from '../utils/fs.utils';
import { CheerioAPI, load } from 'cheerio';

export const replacePublicLinksInHtml = (
  virtualDOM: CheerioAPI,
  pathPrefix: string,
  appAbsoluteUrl: string,
): string => {
  virtualDOM('script').each((_, current) => {
    const elem = virtualDOM(current);
    const src = elem.attr('src');
    if (!src) {
      return;
    }

    if (src.startsWith(appAbsoluteUrl) || elem.data('versionedPath') !== undefined) {
      elem.attr(
        'src',
        src.startsWith(appAbsoluteUrl)
          ? `${appAbsoluteUrl}${pathPrefix}${src.replace(appAbsoluteUrl, '')}`
          : `${pathPrefix}${src}`,
      );
    }
  });

  virtualDOM('link').each((_, current) => {
    const elem = virtualDOM(current);
    const href = elem.attr('href');
    if (elem.attr('rel')?.startsWith('canonical') || !href) {
      return;
    }

    if (href.startsWith(appAbsoluteUrl) || elem.data('versionedPath') !== undefined) {
      elem.attr(
        'href',
        href.startsWith(appAbsoluteUrl)
          ? `${appAbsoluteUrl}${pathPrefix}${href.replace(appAbsoluteUrl, '')}`
          : `${pathPrefix}${href}`,
      );
    }
  });

  virtualDOM('a').each((_, current) => {
    const elem = virtualDOM(current);
    const href = elem.attr('href');
    if (!href) {
      return;
    }

    if (href.startsWith(appAbsoluteUrl) || elem.data('versionedPath') !== undefined) {
      elem.attr(
        'href',
        href.startsWith(appAbsoluteUrl)
          ? `${appAbsoluteUrl}${pathPrefix}${href.replace(appAbsoluteUrl, '')}`
          : `${pathPrefix}${href}`,
      );
    }
  });

  virtualDOM('meta').each((_, current) => {
    const elem = virtualDOM(current);
    const content = elem.attr('content');
    if (!content) {
      return;
    }

    if (content.startsWith(appAbsoluteUrl) || elem.data('versionedPath') !== undefined) {
      elem.attr(
        'content',
        content.startsWith(appAbsoluteUrl)
          ? `${appAbsoluteUrl}${pathPrefix}${content.replace(appAbsoluteUrl, '')}`
          : `${pathPrefix}${content}`,
      );
    }
  });

  return virtualDOM.html();
};

export const withVersionedEntrypoint = (
  outDir = resolve(__dirname, '../../dist'),
  publicDir = resolve(__dirname, '../../public'),
  version = WALLET_VERSION,
  appAbsoluteUrl = ENV.APP_URL,
): Plugin => {
  return {
    name: 'with-versioned-entrypoint',
    writeBundle() {
      const versionDir = resolve(outDir, `v${version}`);
      if (!existsSync(versionDir)) {
        mkdirSync(versionDir);
      }

      const indexHtml = resolve(outDir, 'index.html');
      const indexHtmlContent = readFileSync(indexHtml, { encoding: 'utf-8' });
      const html = replacePublicLinksInHtml(load(indexHtmlContent), `/v${version}`, appAbsoluteUrl);
      writeFileSync(resolve(versionDir, 'index.html'), html, {
        encoding: 'utf-8',
      });

      // copy everything from the public directory to the versioned directory
      copyFolderRecursiveSync(publicDir, versionDir, [/.DS_Store/]);
    },
  };
};
