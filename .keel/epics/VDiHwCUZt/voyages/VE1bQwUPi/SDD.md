# Contract Enforcement - Software Design Description

## Overview

This voyage enforces a strict markdown contract for bearing research documents. By flattening the section hierarchy (moving from ### to ## for key sections), we simplify the parsing logic and ensure that the diagnostic engine can provide deterministic feedback on document completeness.

## Architecture

1.  **Template Realignment**: All bearing templates are updated to the new ## standard.
2.  **Diagnostic Guard**: The `check_bearing_content_sections` logic is extended to validate the existence of ## headings in both EVIDENCE.md and ASSESSMENT.md.
3.  **Projection Update**: The `bearing show` projection is updated to extract content from the new ## boundaries.

## Component Design

### Parsing Logic
We reuse the existing `extract_section` helper, targeting the new top-level headings.

### Health Subsystem
The KINETIC and SENSORY subsystems are updated to reflect the new contract integrity.
