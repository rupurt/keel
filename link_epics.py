import os
import re

mission_id = "VDZKSJa7V"
epics = ["VDZKYMeNQ", "VDZKYQ9RC", "VDZKYTeQK", "VDZKYX7TX"]

for epic_id in epics:
    path = f".keel/epics/{epic_id}/README.md"
    with open(path, 'r') as f:
        content = f.read()
    
    # Add mission field to frontmatter
    content = re.sub(r'title: (.*)\n', r'title: \1\nmission: ' + mission_id + '\n', content)
    
    with open(path, 'w') as f:
        f.write(content)
