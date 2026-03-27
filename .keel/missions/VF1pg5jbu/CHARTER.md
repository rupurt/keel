# Create Formal MDX Documentation Experience For Keel - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Create an onboarding-first public MDX documentation experience for Keel that introduces the model gradually, leads with product narrative, supports persona-specific tracks, and absorbs the existing routine automation guide into the new docs surface. | board: VF1pg68bv |

## Constraints

- The docs must be OSS-facing first and avoid paid-only or hosted-product assumptions in the first public pass.
- Keel terminology should be translated first and introduced gradually rather than front-loading the full internal vocabulary.
- Examples must remain AI-vendor-neutral and work across different human/AI harnesses.
- The site must build to static assets suitable for deployment behind S3 and CloudFront on `spoke.sh`.

## Halting Rules

- DO NOT halt while Keel still lacks a formal public MDX docs surface for onboarding external OSS users.
- HALT when the docs site scaffolding, core narrative pages, persona tracks, and migrated routines guidance are all landed and the board work is closed.
- YIELD to human if the first-pass docs require product or commercial positioning beyond the OSS-facing scope.
