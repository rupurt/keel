# PRESS RELEASE: Output Contract And Shared Renderer

## Overview

## Narrative Summary
### Implement Shared Guidance Renderer Helpers
Extract and reuse shared guidance rendering helpers so commands emit canonical next and recovery guidance through one implementation path.

### Define Canonical Guidance Output Contract
Define a canonical command-guidance contract that can represent one deterministic next step or one deterministic recovery step, and wire it into `keel next` JSON output as the baseline for broader command adoption.

### Add Contract Tests For Canonical Guidance Fields
Add regression tests that lock the canonical guidance payload shape so downstream harnesses can rely on stable `next_step` and `recovery_step` semantics.

## Key Insights
## Verification Proof
### Proof for Implement Shared Guidance Renderer Helpers
- [ac-1.log](../../../../stories/1vxZ0BinH/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0BinH/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0BinH/EVIDENCE/ac-2.log)

### Proof for Define Canonical Guidance Output Contract
- [ac-1.log](../../../../stories/1vxZ0AFJK/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0AFJK/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0AFJK/EVIDENCE/ac-2.log)

### Proof for Add Contract Tests For Canonical Guidance Fields
- [ac-1.log](../../../../stories/1vxZ0Bh0v/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0Bh0v/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0Bh0v/EVIDENCE/ac-2.log)

