import os
import re

files = [
    "src/application/story_lifecycle.rs",
    "src/application/voyage_epic_lifecycle.rs",
]

for file_path in files:
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Add imports to the test module
    if "#[cfg(test)]" in content:
        test_imports = [
            "use crate::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;",
            "use crate::application::process_manager::{DomainProcessManager, LiveProcessActionExecutor};",
        ]
        for imp in test_imports:
            if imp not in content:
                content = content.replace("mod tests {", "mod tests {\n    " + imp)

    with open(file_path, 'w') as f:
        f.write(content)
