import os
import re

stories = ["VDZb97pMO", "VDZb9BcPN", "VDZb9FeSU", "VDZb9JXUt", "VDZb9NTVh", "VDZb9RDYg"]

for story_id in stories:
    path = f".keel/stories/{story_id}/README.md"
    if not os.path.exists(path):
        continue
    with open(path, 'r') as f:
        content = f.read()
    
    if "SRS-01:start:end" not in content and "SRS-01" in content:
        # replace any SRS-01 marker with start:end
        content = re.sub(r'SRS-01:[a-z:]+', 'SRS-01:start:end', content)
    
    with open(path, 'w') as f:
        f.write(content)
