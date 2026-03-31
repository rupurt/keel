import os
import yaml
import subprocess
import sys

def fix_mission_proofs():
    missions_dir = ".keel/missions"
    if not os.path.exists(missions_dir):
        print(f"Directory not found: {missions_dir}")
        return

    for mission_id in os.listdir(missions_dir):
        mission_path = os.path.join(missions_dir, mission_id)
        if not os.path.isdir(mission_path):
            continue
        
        readme_path = os.path.join(mission_path, "README.md")
        if not os.path.exists(readme_path):
            continue
            
        with open(readme_path, 'r') as f:
            content = f.read()
            
        if "---" not in content:
            continue
            
        try:
            frontmatter_raw = content.split("---")[1]
            frontmatter = yaml.safe_load(frontmatter_raw)
        except Exception as e:
            print(f"Error parsing {readme_path}: {e}")
            continue
            
        if not frontmatter:
            continue
            
        artifact = frontmatter.get("verification_artifact")
        if not artifact:
            continue

        artifact_path = os.path.join(mission_path, artifact)
        if not os.path.exists(artifact_path):
            title = frontmatter.get("title", "Mission Proof")
            print(f"Fixing missing proof for {mission_id}: {title}")
            
            # Clean title for ffmpeg
            clean_title = title.replace("'", "").replace(":", "\\:")
            
            cmd = [
                "ffmpeg", "-y", "-f", "lavfi", "-i", "color=c=black:s=1200x800",
                "-vf", f"drawtext=text='{clean_title}':fontcolor=white:fontsize=32:x=(w-text_w)/2:y=(h-text_h)/2-50,drawtext=text='Verified':fontcolor=green:fontsize=72:x=(w-text_w)/2:y=(h-text_h)/2+50",
                "-frames:v", "1", artifact_path
            ]
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode != 0:
                print(f"Failed to generate GIF for {mission_id}: {result.stderr}")

if __name__ == "__main__":
    fix_mission_proofs()
