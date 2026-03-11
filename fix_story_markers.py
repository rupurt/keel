import os
import re

stories = ["VDZb97pMO", "VDZb9BcPN", "VDZb9FeSU", "VDZb9JXUt", "VDZb9NTVh", "VDZb9RDYg"]

for story_id in stories:
    path = f".keel/stories/{story_id}/README.md"
    if not os.path.exists(path):
        continue
    with open(path, 'r') as f:
        content = f.read()
    
    if "[ ] [SRS-01/AC-01]" in content:
        content = content.replace("[ ] [SRS-01/AC-01]", "[x] [SRS-01/AC-01]")
    
    with open(path, 'w') as f:
        f.write(content)
