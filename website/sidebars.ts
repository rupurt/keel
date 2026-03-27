import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docsSidebar: [
    'intro',
    {
      type: 'category',
      label: 'Start Here',
      items: ['start-here/install-keel', 'start-here/first-turn'],
    },
    {
      type: 'category',
      label: 'Foundations',
      items: [
        'foundations/board-model',
        'foundations/roles-and-lanes',
        'foundations/planning-and-verification',
      ],
    },
    {
      type: 'category',
      label: 'Workflows',
      items: [
        'workflows/everyday-keel',
        'workflows/downstream-project-contracts',
        'workflows/upgrading-keel-and-syncing-instructions',
        'workflows/routines-and-pulse',
      ],
    },
    {
      type: 'category',
      label: 'Persona Tracks',
      items: [
        'personas/project-managers',
        'personas/programmers',
        'personas/designers',
        'personas/leaders-and-specialists',
      ],
    },
  ],
};

export default sidebars;
