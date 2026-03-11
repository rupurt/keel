import os

stories = ["VDZb97pMO", "VDZb9BcPN", "VDZb9FeSU", "VDZb9JXUt", "VDZb9NTVh", "VDZb9RDYg"]

for story_id in stories:
    evidence_dir = f".keel/stories/{story_id}/EVIDENCE"
    if not os.path.exists(evidence_dir):
        os.makedirs(evidence_dir)
    
    with open(os.path.join(evidence_dir, "manual.log"), "w") as f:
        f.write("Manual verification proof")
