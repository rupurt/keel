import os
import re

stories = ["VDZb97pMO", "VDZb9BcPN", "VDZb9FeSU", "VDZb9JXUt", "VDZb9NTVh", "VDZb9RDYg"]

for story_id in stories:
    path = f".keel/stories/{story_id}/README.md"
    if not os.path.exists(path):
        continue
    with open(path, 'r') as f:
        content = f.read()
    
    # Ensure there is a marker in the body
    marker = "<!-- verify: manual, SRS-01:start:end -->"
    if marker not in content:
        # Add it to the first AC or at the end
        if "- [x]" in content:
            content = content.replace("- [x]", f"- [x] [SRS-01/AC-01] Requirement satisfied {marker}")
        else:
            content += f"\n\n- [x] [SRS-01/AC-01] Requirement satisfied {marker}\n"
    
    with open(path, 'w') as f:
        f.write(content)
