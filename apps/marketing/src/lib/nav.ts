import { docsUrl, walletUrl } from './config';

export const headerNav = {
  navigation: [
    {
      label: 'Features',
      href: '#features',
    },
    {
      label: 'Products',
      href: '#products',
    },
    {
      label: 'FAQ',
      href: '#faq',
    },
    {
      label: 'Clients',
      href: '#clients',
    },
  ],
};

export const footerNav = {
  navigation: [
    {
      label: 'Features',
      href: '#features',
    },
    {
      label: 'Products',
      href: '#products',
    },

    {
      label: 'FAQ',
      href: '#faq',
    },
    {
      label: 'Clients',
      href: '#clients',
    },
  ],
  orbit: [
    {
      label: 'Contact',
      href: 'mailto:orbit@dfinity.org',
    },
    {
      label: 'Wallet',
      href: walletUrl,
    },
    {
      label: 'Documentation',
      href: docsUrl,
    },
  ],
};

export const footerLinks = {
  other: [
    {
      label: 'License',
      href: '/license',
    },
    {
      label: 'Change log',
      href: 'https://github.com/dfinity/orbit/releases',
    },
  ],
  social: [
    {
      label: 'Twitter',
      href: 'https://x.com/dfinity',
    },
    {
      label: 'Github',
      href: 'https://github.com/dfinity/orbit',
    },
  ],
};
