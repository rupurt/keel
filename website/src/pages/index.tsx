import type {ReactNode} from 'react';
import Link from '@docusaurus/Link';
import Layout from '@theme/Layout';
import SignalGrid from '@site/src/components/SignalGrid';
import TurnCycle from '@site/src/components/TurnCycle';
import TermRamp from '@site/src/components/TermRamp';
import PersonaGrid from '@site/src/components/PersonaGrid';

import styles from './index.module.css';

const signalItems = [
  {
    eyebrow: 'Board Discipline',
    title: 'Reduce drift before it spreads',
    body:
      'Keel treats planning gaps, stale state, and missing evidence as first-class system problems instead of project folklore.',
    href: '/docs/foundations/board-model',
    cta: 'Read the board model',
  },
  {
    eyebrow: 'Shared Operations',
    title: 'Give humans and agents one operating surface',
    body:
      'The board, the CLI, and the planning artifacts form one public contract that works across terminals, editors, and AI harnesses.',
    href: '/docs/workflows/downstream-project-contracts',
    cta: 'See the operating contract',
  },
  {
    eyebrow: 'Proof Over Vibes',
    title: 'Close work with evidence',
    body:
      'Stories do not end when someone says they are done. They end when the board can show why they should be trusted.',
    href: '/docs/foundations/planning-and-verification',
    cta: 'Learn verification flow',
  },
];

const laneItems = [
  {
    eyebrow: 'Management Lane',
    title: 'Choose direction',
    body:
      'Use missions, epics, voyages, and bearings to decide what the team should do next and what constraints matter.',
  },
  {
    eyebrow: 'Delivery Lane',
    title: 'Ship one slice at a time',
    body:
      'Pull work with explicit roles, move it through clear states, and keep every turn small enough to verify.',
  },
];

export default function Home(): ReactNode {
  return (
    <Layout
      title="Keel"
      description="Keel is a turn-based board engine for human/AI delivery teams.">
      <main className={styles.page}>
        <section className={styles.hero}>
          <div className="container">
            <div className={styles.heroGrid}>
              <div className={styles.heroCopy}>
                <p className={styles.eyebrow}>Agentic Board Engine</p>
                <h1>Operate delivery one turn at a time.</h1>
                <p className={styles.lede}>
                  Keel is a turn-based board engine for human/AI delivery
                  teams. It turns planning, execution, and verification into
                  one shared board with explicit moves, role-aware lanes, and
                  evidence-backed closure.
                </p>
                <div className={styles.actions}>
                  <Link className={styles.primaryAction} to="/docs/intro">
                    Read The Story
                  </Link>
                  <Link
                    className={styles.secondaryAction}
                    to="/docs/start-here/install-keel">
                    Install Keel
                  </Link>
                </div>
                <ul className={styles.heroPoints}>
                  <li>Translate from plain language into Keel terms gradually.</li>
                  <li>Keep examples neutral across editors, terminals, and AI tools.</li>
                  <li>Use one board for planning, delivery, routines, and verification.</li>
                </ul>
              </div>
              <div className={styles.scenePanel}>
                <div className={styles.sceneFrame}>
                  <div className={styles.sceneChrome} aria-hidden="true">
                    <span />
                    <span />
                    <span />
                  </div>
                  <p className={styles.sceneLabel}>Typical First Turn</p>
                  <ol className={styles.sceneSteps}>
                    <li>
                      <span>Orient</span>
                      <code>keel health --scene</code>
                    </li>
                    <li>
                      <span>Inspect</span>
                      <code>keel mission next --status</code>
                    </li>
                    <li>
                      <span>Pull</span>
                      <code>keel next --role operator</code>
                    </li>
                    <li>
                      <span>Verify</span>
                      <code>keel doctor</code>
                    </li>
                  </ol>
                </div>
              </div>
            </div>
          </div>
        </section>

        <section className={styles.section}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <p className={styles.sectionEyebrow}>Why Teams Reach For Keel</p>
              <h2>Keel behaves like an operating model, not a notes app.</h2>
              <p>
                The board is opinionated on purpose. It creates a visible system
                for planning, work intake, delivery, and verification so humans
                and AI collaborators stop improvising different versions of
                reality.
              </p>
            </div>
            <SignalGrid items={signalItems} />
          </div>
        </section>

        <section className={styles.sectionAlt}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <p className={styles.sectionEyebrow}>The Turn Loop</p>
              <h2>A ticket can be vague. A turn has a visible state change.</h2>
              <p>
                Keel keeps the loop legible: orient the board, pick the next
                move, execute the slice, and close it with evidence before the
                next turn starts.
              </p>
            </div>
            <TurnCycle />
          </div>
        </section>

        <section className={styles.section}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <p className={styles.sectionEyebrow}>Native Vocabulary</p>
              <h2>Start in plain language, then learn the board terms.</h2>
              <p>
                You do not need to memorize Keel jargon on day one. The docs
                introduce each term only after the underlying job is already
                clear.
              </p>
            </div>
            <TermRamp />
          </div>
        </section>

        <section className={styles.sectionAlt}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <p className={styles.sectionEyebrow}>Lanes And Roles</p>
              <h2>Strategy and delivery stay connected without collapsing.</h2>
              <p>
                Keel separates direction-setting from implementation execution
                so teams can move quickly without losing authority, review, or
                context.
              </p>
            </div>
            <SignalGrid items={laneItems} columns="two" />
          </div>
        </section>

        <section className={styles.section}>
          <div className="container">
            <div className={styles.sectionHeader}>
              <p className={styles.sectionEyebrow}>Persona Tracks</p>
              <h2>Learn the basics once, then follow the path that matches your job.</h2>
              <p>
                After the shared onboarding, Keel branches into role-focused
                guidance for builders, operators, and leaders who need to read
                the same board from different angles.
              </p>
            </div>
            <PersonaGrid />
          </div>
        </section>

        <section className={styles.ctaBand}>
          <div className="container">
            <div className={styles.ctaCard}>
              <div>
                <p className={styles.sectionEyebrow}>Start Here</p>
                <h2>Read the narrative, install the CLI, and take the first turn.</h2>
              </div>
              <div className={styles.actions}>
                <Link className={styles.primaryAction} to="/docs/intro">
                  Open The Docs
                </Link>
                <Link
                  className={styles.secondaryAction}
                  to="/docs/start-here/first-turn">
                  First Turn Guide
                </Link>
              </div>
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
