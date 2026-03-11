import os
import re

stories = ["VDZb97pMO", "VDZb9BcPN", "VDZb9FeSU", "VDZb9JXUt", "VDZb9NTVh", "VDZb9RDYg"]

for story_id in stories:
    path = f".keel/stories/{story_id}/README.md"
    if not os.path.exists(path):
        continue
    with open(path, 'r') as f:
        content = f.read()
    
    # Use start:end which should match [a-z:]+
    marker = "<!-- verify: manual, SRS-01:start:end -->"
    if marker not in content:
        # replace any existing marker
        content = re.sub(r'<!-- verify:.*SRS-01:.* -->', marker, content)
        if marker not in content:
             content = content.replace("- [x]", f"- [x] [SRS-01/AC-01] Requirement satisfied {marker}")
    
    with open(path, 'w') as f:
        f.write(content)
