import type {ReactNode} from 'react';
import Head from '@docusaurus/Head';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import {useDoc} from '@docusaurus/plugin-content-docs/client';
import {PageMetadata, useThemeConfig} from '@docusaurus/theme-common';

export default function DocItemMetadata(): ReactNode {
  const {metadata, frontMatter, assets} = useDoc();
  const {title: siteTitle, titleDelimiter = '|'} =
    useDocusaurusContext().siteConfig;
  const {image: defaultImage} = useThemeConfig();

  const image = assets.image ?? frontMatter.image ?? defaultImage;
  const socialTitle = metadata.title
    ? `${metadata.title} ${titleDelimiter} ${siteTitle}`
    : siteTitle;

  return (
    <>
      <PageMetadata
        title={metadata.title}
        description={metadata.description}
        keywords={frontMatter.keywords}
        image={image}
      />
      <Head>
        <meta property="og:type" content="article" />
        <meta property="og:site_name" content={siteTitle} />
        <meta name="twitter:title" content={socialTitle} />
        {metadata.description && (
          <meta name="twitter:description" content={metadata.description} />
        )}
      </Head>
    </>
  );
}
