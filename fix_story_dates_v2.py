import os
import re
from datetime import datetime, timedelta

stories = ["VDZb97pMO", "VDZb9BcPN", "VDZb9FeSU", "VDZb9JXUt", "VDZb9NTVh", "VDZb9RDYg"]
now = datetime.now()

for i, story_id in enumerate(stories):
    path = f".keel/stories/{story_id}/README.md"
    if not os.path.exists(path):
        continue
    with open(path, 'r') as f:
        content = f.read()
    
    # Sequence the dates for manual accept stories:
    # created < started < submitted < completed
    created = (now - timedelta(hours=4)).strftime("%Y-%m-%dT%H:%M:%S")
    started = (now - timedelta(hours=3)).strftime("%Y-%m-%dT%H:%M:%S")
    submitted = (now - timedelta(hours=2)).strftime("%Y-%m-%dT%H:%M:%S")
    completed = (now - timedelta(hours=1)).strftime("%Y-%m-%dT%H:%M:%S")
    
    content = re.sub(r'created_at: .*\n', f'created_at: {created}\n', content)
    content = re.sub(r'started_at: .*\n', f'started_at: {started}\n', content)
    content = re.sub(r'submitted_at: .*\n', f'submitted_at: {submitted}\n', content)
    content = re.sub(r'completed_at: .*\n', f'completed_at: {completed}\n', content)
    
    with open(path, 'w') as f:
        f.write(content)
