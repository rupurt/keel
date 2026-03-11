import os
import re

for root, dirs, files in os.walk(".keel/stories"):
    if "README.md" in files:
        path = os.path.join(root, "README.md")
        with open(path, 'r') as f:
            content = f.read()
        
        # Remove empty AC items
        content = re.sub(r'- \[ \] \n', '', content)
        content = re.sub(r'- \[ \]\s*\n', '', content)
        
        # Consolidate double headers
        content = re.sub(r'## Acceptance Criteria\n\n## Acceptance Criteria', '## Acceptance Criteria', content)
        
        # Final cleanup of excessive newlines
        content = re.sub(r'\n{3,}', '\n\n', content)
        
        with open(path, 'w') as f:
            f.write(content)
