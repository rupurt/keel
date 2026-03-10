#!/usr/bin/env bash
set -euo pipefail

story_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)"
repo_root="$(cd "$story_dir"/../../.. && pwd)"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

mkdir -p \
  "$tmp_dir/.keel/stories/RVW0001" \
  "$tmp_dir/.keel/stories/DEL0001" \
  "$tmp_dir/.keel/stories/WIP0001" \
  "$tmp_dir/.keel/stories/DON0001" \
  "$tmp_dir/.keel/epics/demo-topology/voyages/01-review" \
  "$tmp_dir/.keel/bearings/market-scan"

cat >"$tmp_dir/keel.toml" <<'EOF'
[workflow.defaults]
management_role = "reviewer"
delivery_role = "maker"
management_lane = "review"
delivery_lane = "delivery"

[roles.reviewer]
default_lane = "review"
template = "reviewer-core"

[roles.maker]
default_lane = "delivery"
template = "maker-core"

[roles.researcher]
default_lane = "research"
template = "researcher-core"

[lanes.review]
description = "Manual review work"
include = ["story.needs-human-verification", "voyage.draft"]
exclude = []
parallel = false
manual_accept = true
priority = 300

[lanes.delivery]
description = "Delivery work"
include = ["story.*"]
exclude = ["story.done", "story.icebox", "story.needs-human-verification", "story.rejected"]
parallel = true
manual_accept = false
priority = 200

[lanes.research]
description = "Research work"
include = ["bearing.exploring"]
exclude = []
parallel = false
manual_accept = false
priority = 100
EOF

cat >"$tmp_dir/.keel/epics/demo-topology/README.md" <<'EOF'
---
id: demo-topology
title: Demo Topology
---
# Demo Topology
EOF

cat >"$tmp_dir/.keel/epics/demo-topology/voyages/01-review/README.md" <<'EOF'
---
id: 01-review
title: Review Lane Voyage
status: draft
epic: demo-topology
---
# Review Lane Voyage
EOF

cat >"$tmp_dir/.keel/bearings/market-scan/README.md" <<'EOF'
---
id: market-scan
title: Market Scan
status: exploring
---
# Market Scan
EOF

cat >"$tmp_dir/.keel/stories/RVW0001/README.md" <<'EOF'
---
id: RVW0001
title: Review queued work
type: feat
status: needs-human-verification
scope: demo-topology/01-review
---
# Review queued work
EOF

cat >"$tmp_dir/.keel/stories/DEL0001/README.md" <<'EOF'
---
id: DEL0001
title: Delivery backlog slice
type: feat
status: backlog
scope: demo-topology/01-review
---
# Delivery backlog slice
EOF

cat >"$tmp_dir/.keel/stories/WIP0001/README.md" <<'EOF'
---
id: WIP0001
title: Delivery in-flight slice
type: feat
status: in-progress
scope: demo-topology/01-review
---
# Delivery in-flight slice
EOF

cat >"$tmp_dir/.keel/stories/DON0001/README.md" <<'EOF'
---
id: DON0001
title: Excluded done slice
type: feat
status: done
scope: demo-topology/01-review
---
# Excluded done slice
EOF

cd "$tmp_dir"
if [[ -t 0 ]]; then
  stty cols 100 rows 28
fi
cargo run --quiet --manifest-path "$repo_root/Cargo.toml" -- flow --no-color
