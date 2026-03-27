import SignalGrid from '@site/src/components/SignalGrid';

const items = [
  {
    eyebrow: 'Project Managers',
    title: 'Drive planning without losing delivery reality',
    body:
      'Learn how to create clean arcs, manage lanes, and pull the right signals before asking the team for another move.',
    href: '/docs/personas/project-managers',
    cta: 'Open the PM track',
  },
  {
    eyebrow: 'Programmers',
    title: 'Turn code changes into board-safe slices',
    body:
      'Use stories, evidence, verification, and routines without needing a separate system for engineering coordination.',
    href: '/docs/personas/programmers',
    cta: 'Open the programmer track',
  },
  {
    eyebrow: 'Designers',
    title: 'Shape the board from discovery to acceptance',
    body:
      'Work through bearings, review loops, and story evidence so design work stays visible and enforceable.',
    href: '/docs/personas/designers',
    cta: 'Open the designer track',
  },
  {
    eyebrow: 'Leaders And Specialists',
    title: 'Read the system without living in it all day',
    body:
      'Marketers, lawyers, general managers, and executives can use the same board to review scope, risk, and readiness.',
    href: '/docs/personas/leaders-and-specialists',
    cta: 'Open the leadership track',
  },
];

export default function PersonaGrid() {
  return <SignalGrid items={items} columns="two" />;
}
