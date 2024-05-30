import { CheerioAPI, load } from 'cheerio';
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'fs';
import { join, resolve } from 'path';
import { Plugin } from 'vite';
import { ENV, WALLET_VERSION } from '../core/configs.core';
import { copyFolderRecursiveSync } from '../utils/fs.utils';

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
  basePath = '/',
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
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!req.url || !req.headers.host) {
          return next();
        }

        const url = new URL(`http://${req.headers.host}${req.url}`);
        if (url.pathname.startsWith(basePath)) {
          url.pathname = url.pathname.slice(basePath.length);
        }

        const pathParts = url.pathname.split('/').filter(Boolean);
        // TODO: update to use semantic version check
        if (!pathParts?.[0]?.startsWith('v')) {
          return next();
        }

        const unversionedPath = pathParts.slice(1).join('/');
        const unversionedUrl = new URL(url.href);
        unversionedUrl.pathname = unversionedPath;

        const filePath = join(publicDir, unversionedUrl.pathname);
        if (existsSync(filePath) && statSync(filePath).isFile()) {
          return res
            .writeHead(307, {
              Location: unversionedUrl.href,
            })
            .end();
        }

        console.warn(
          `\x1b[33m[warning] Versioned path during dev.
          This could happen if you connect to a station canister that is still on a previous version.
          To test the navigation across different versions, use the vite preview command and add the versions in the dist folder.\x1b[0m`,
        );

        next();
      });
    },
    configurePreviewServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!req.url || !req.headers.host) {
          return next();
        }

        const url = new URL(`http://${req.headers.host}${req.url}`);
        if (url.pathname.startsWith(basePath)) {
          url.pathname = url.pathname.slice(basePath.length);
        }

        const pathParts = url.pathname.split('/').filter(Boolean);
        // TODO: update to use semantic version check
        if (!pathParts?.[0]?.startsWith('v')) {
          return next();
        }

        const requestedVersion = pathParts[0].replace('v', '');
        const versionDistFolder = join(outDir, `v${requestedVersion}`);

        if (!existsSync(versionDistFolder)) {
          return res
            .writeHead(404, {
              'Content-Type': 'text/plain',
            })
            .end(
              `APP Version requested \`${requestedVersion}\` not found.\n\nTo emulate a build with multiple previous versions, add them to the dist folder and use the vite preview command.`,
            );
        }

        // return to the user the index.html of the versioned folder, required for SPA routing on vite preview mode
        const filePath = join(outDir, url.pathname);
        if (!existsSync(filePath) || !statSync(filePath).isFile()) {
          const indexHtml = join(versionDistFolder, 'index.html');
          if (!existsSync(indexHtml)) {
            return res
              .writeHead(404, { 'Content-Type': 'text/plain' })
              .end('Versioned index.html not found.');
          }

          const indexHtmlContent = readFileSync(indexHtml, { encoding: 'utf-8' });

          return res
            .writeHead(200, {
              'Content-Type': 'text/html',
            })
            .end(indexHtmlContent);
        }

        return next();
      });
    },
  };
};
