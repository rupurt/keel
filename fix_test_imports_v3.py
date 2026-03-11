import os
import re

diag_dir = "src/read_model/diagnostics/checks"
for root, dirs, files in os.walk(diag_dir):
    for file in files:
        if file.endswith(".rs"):
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                content = f.read()
            
            # Remove unused validate import in tests
            content = content.replace("    use crate::read_model::diagnostics::validate;\n", "")
            
            # Remove specific evidence.rs unused imports
            if file == "evidence.rs":
                content = content.replace("        EnforcementPolicy, TransitionEntity, TransitionIntent, VoyageTransition, enforce_transition,\n", "")

            with open(path, 'w') as f:
                f.write(content)

# Fix epic/new.rs
epic_new = "src/cli/commands/management/epic/new.rs"
with open(epic_new, 'r') as f:
    content = f.read()
content = content.replace("    use crate::cli::commands::diagnostics::doctor;\n", "")
with open(epic_new, 'w') as f:
    f.write(content)

