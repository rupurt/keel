# Documentation Theme

The documentation theme is a paper-neumorphic system with a fixed lighting model.

## Visual Thesis

The docs should feel like:

- paper as the base plane
- raised nautical instruments and controls above that paper
- crisp industrial restraint in the spirit of Teenage Engineering, Apple, and OpenAI

That means:

- simple geometry
- careful spacing
- restrained color
- visible elevation without glossy effects
- mechanical, legible controls instead of decorative flourishes

## Light Physics

Assume the light source sits at the top-right of the page.

Implications:

- raised objects cast shadows toward the bottom-left
- lit edges should read more clearly on the top and right faces
- the page paper itself should not cast a floating shadow
- inset areas should feel recessed into paper, not like separate cards

## Planes

There are three planes in the docs theme:

1. Shell
   The navbar and major control surfaces float above the page and can cast the strongest shadow.
2. Paper
   The page background is the resting surface. It should be visually calm and should not look elevated.
3. Raised Components
   Cards, buttons, pills, and framed modules sit above paper with one consistent directional shadow system.

## Component Rules

- Header:
  Shadow belongs on the blue shell, not on the spacer paper beneath it.
- Page background:
  Use a flat eggshell paper tone with no external depth effect.
- Cards and panels:
  Use the shared raised shadow tokens and avoid ad hoc ambient glows.
- Buttons and pills:
  Should feel like nautical controls: compact, tactile, and slightly lifted from the page.
- Recessed surfaces:
  Terminals and internal trays should rely on inset contrast rather than external drop shadows.

## Motion

Motion in the docs should be structural, not ornamental.

Rules:

- motion should guide reading order, continuity, or state change
- ambient animation should be slow, sparse, and staggered
- moving surfaces must still obey the lighting model instead of inventing a second visual system
- all non-essential motion must shut off under `prefers-reduced-motion`

Turnsteps are the canonical motion pattern:

- use 2-4 hex steps between sections
- animate them as a traveling cadence, not a bounce loop
- one step should crest at a time so the eye reads progression through the page
- the motion should feel like footsteps through paper or sand, with a slight rise and settle rather than a jump
- opacity and inset highlight may pulse with the active step, but the overall surface must remain calm
- pointer devices may exert local gravity so nearby steps scale up as the cursor approaches
- gravity should feather in at the perimeter, then tighten so the nearest step grows most once the pointer is inside the trail
- the gravity field must stay local to the trail, decay quickly, and drop out on touch or reduced-motion contexts

## Avoid

- symmetrical ambient shadows that ignore the light source
- gradients as surface decoration
- paper surfaces with conflicting elevations
- components inventing their own shadow language
