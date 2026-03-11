import os

voyages = [
    ".keel/epics/VDZKYMeNQ/voyages/VDZb48rCW/SRS.md",
    ".keel/epics/VDZKYQ9RC/voyages/VDZb4CeET/SRS.md",
    ".keel/epics/VDZKYTeQK/voyages/VDZb4GDHO/SRS.md",
    ".keel/epics/VDZKYX7TX/voyages/VDZb4JrIq/SRS.md",
    ".keel/epics/VDZcE0Uo5/voyages/VDZb48rCW/SRS.md",
    ".keel/epics/VDZcE46pb/voyages/VDZb4CeET/SRS.md",
    ".keel/epics/VDZcE7gsS/voyages/VDZb4GDHO/SRS.md",
]

for path in voyages:
    if os.path.exists(path):
        with open(path, 'r') as f:
            content = f.read()
        if "NON_FUNCTIONAL_REQUIREMENTS" not in content:
            content += "\n## Non-Functional Requirements\n\n<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->\n<!-- END NON_FUNCTIONAL_REQUIREMENTS -->\n"
            with open(path, 'w') as f:
                f.write(content)
