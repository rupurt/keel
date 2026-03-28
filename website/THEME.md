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

## Avoid

- symmetrical ambient shadows that ignore the light source
- gradients as surface decoration
- paper surfaces with conflicting elevations
- components inventing their own shadow language
