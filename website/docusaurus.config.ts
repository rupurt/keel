import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const siteUrl = process.env.DOCS_SITE_URL ?? 'https://spoke.sh';
const baseUrl = process.env.DOCS_BASE_URL ?? '/';
const repoUrl = 'https://github.com/spoke-sh/keel';

const config: Config = {
  title: 'Keel',
  tagline: 'Turn-based board operating engine for human/AI delivery teams',
  favicon: 'img/favicon.svg',
  future: {
    v4: true,
  },
  url: siteUrl,
  baseUrl,
  organizationName: 'spoke-sh',
  projectName: 'keel',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  markdown: {
    hooks: {
      onBrokenMarkdownLinks: 'throw',
    },
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          editUrl: `${repoUrl}/tree/main/website/`,
          routeBasePath: 'docs',
          showLastUpdateAuthor: false,
          showLastUpdateTime: true,
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/keel-social-card.svg',
    colorMode: {
      defaultMode: 'light',
      disableSwitch: true,
      respectPrefersColorScheme: false,
    },
    navbar: {
      title: 'Keel',
      logo: {
        alt: 'Keel logo',
        src: 'img/logo.svg',
      },
      items: [
        {
          type: 'doc',
          docId: 'intro',
          position: 'left',
          label: 'Docs',
        },
        {
          to: '/docs/personas/project-managers',
          label: 'Personas',
          position: 'left',
        },
        {
          href: repoUrl,
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Start Here',
          items: [
            {
              label: 'Why Keel',
              to: '/docs/intro',
            },
            {
              label: 'Install Keel',
              to: '/docs/start-here/install-keel',
            },
            {
              label: 'Take Your First Turn',
              to: '/docs/start-here/first-turn',
            },
          ],
        },
        {
          title: 'Workflows',
          items: [
            {
              label: 'Board Model',
              to: '/docs/foundations/board-model',
            },
            {
              label: 'Planning And Verification',
              to: '/docs/foundations/planning-and-verification',
            },
            {
              label: 'Routines And Pulse',
              to: '/docs/workflows/routines-and-pulse',
            },
          ],
        },
        {
          title: 'Project',
          items: [
            {
              label: 'GitHub',
              href: repoUrl,
            },
            {
              label: 'Architecture',
              href: `${repoUrl}/blob/main/ARCHITECTURE.md`,
            },
            {
              label: 'Policy',
              href: `${repoUrl}/blob/main/POLICY.md`,
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Keel contributors.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
