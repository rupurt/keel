import os
import re

# 1. Update Mission Charter
charter_path = ".keel/missions/VDZKSJa7V/CHARTER.md"
with open(charter_path, 'r') as f:
    charter = f.read()
charter = charter.replace("board: VDZMNiZPR", "board: VDZKYMeNQ")
charter = charter.replace("board: VDZMNZPFG", "board: VDZKYQ9RC")
charter = charter.replace("board: VDZMNYPFG", "board: VDZKYTeQK")
charter = charter.replace("board: VDZMNXPFG", "board: VDZKYX7TX")
with open(charter_path, 'w') as f:
    f.write(charter)

# 2. Set all Epics to done (implicitly through voyages)
# Actually, Epics calculate status from voyages.
# Let's set all Voyages to done.
voyage_readmes = [
    ".keel/epics/VDZKYMeNQ/voyages/VDZb48rCW/README.md",
    ".keel/epics/VDZKYQ9RC/voyages/VDZb4CeET/README.md",
    ".keel/epics/VDZKYTeQK/voyages/VDZb4GDHO/README.md",
    ".keel/epics/VDZKYX7TX/voyages/VDZb4JrIq/README.md",
]

for path in voyage_readmes:
    if os.path.exists(path):
        with open(path, 'r') as f:
            content = f.read()
        content = content.replace("status: draft", "status: done")
        with open(path, 'w') as f:
            f.write(content)

