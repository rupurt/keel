//! `keel play` command — invite play-driven discovery

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use super::play_guidance::{guidance_for_suggest, informational_for_exploration, print_human};

/// Run the play command
pub fn run(
    board_dir: &Path,
    bearing_id: Option<String>,
    prop: Option<String>,
    cross: Option<Vec<String>>,
    list_props: bool,
    suggest: Option<String>,
) -> Result<()> {
    let play_dir = board_dir.join("play");

    let guidance = if let Some(ids) = cross {
        if ids.len() != 2 {
            bail!("`--cross` requires exactly two bearing IDs");
        }
        run_cross(board_dir, &ids[0], &ids[1])?;
        informational_for_exploration()
    } else if let Some(ref bearing) = suggest {
        let recommended_prop = run_suggest(board_dir, bearing)?;
        guidance_for_suggest(bearing, &recommended_prop)
    } else if list_props {
        list_available_props(&play_dir)?;
        informational_for_exploration()
    } else if let Some(ref bearing) = bearing_id {
        play_bearing(board_dir, &play_dir, bearing, prop.as_deref())?;
        informational_for_exploration()
    } else {
        // Freeform play
        freeform_play(&play_dir, prop.as_deref())?;
        informational_for_exploration()
    };

    print_human(guidance.as_ref());
    Ok(())
}

/// List all available props by category
fn list_available_props(play_dir: &Path) -> Result<()> {
    let props_dir = play_dir.join("props");

    if !props_dir.exists() {
        println!("No props catalog found. Create one at .keel/play/props/");
        return Ok(());
    }

    println!("🎭 Props (reframing tools)\n");

    let mut found_any = false;
    let mut categories: Vec<_> = fs::read_dir(&props_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    categories.sort_by_key(|e| e.file_name());

    for category_entry in categories {
        let category_name = category_entry.file_name();
        let category_name = category_name.to_string_lossy();

        let mut props: Vec<String> = fs::read_dir(category_entry.path())?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
            .map(|e| {
                e.path()
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();
        props.sort();

        if !props.is_empty() {
            found_any = true;
            let label = match category_name.as_ref() {
                "masks" => "Masks (perspective shifts)",
                "hats" => "Hats (thinking modes)",
                "instruments" => "Instruments (tempo/energy)",
                "costumes" => "Costumes (context shifts)",
                _ => &category_name,
            };
            println!("  {}:", label);

            for prop in &props {
                // Read first line of description from the prop file
                let prop_path = category_entry.path().join(format!("{}.md", prop));
                let desc = read_prop_tagline(&prop_path);
                println!("    {:<14} — {}", prop, desc);
            }
            println!();
        }
    }

    if !found_any {
        println!("  No props found. Add .md files to .keel/play/props/<category>/");
    }

    Ok(())
}

/// Read the "Reframes by" tagline from a prop file
fn read_prop_tagline(path: &Path) -> String {
    let content = fs::read_to_string(path).unwrap_or_default();
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("**Reframes by:**") {
            return rest.trim().to_string();
        }
    }
    "a reframing tool".to_string()
}

/// Read the core prompt from a prop file
fn read_prop_prompt(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let mut in_core_prompt = false;
    for line in content.lines() {
        if line.starts_with("## Core Prompt") {
            in_core_prompt = true;
            continue;
        }
        if in_core_prompt {
            let trimmed = line.trim();
            if trimmed.starts_with("> ") {
                return Some(trimmed.strip_prefix("> ").unwrap_or(trimmed).to_string());
            }
            if trimmed.starts_with('#') {
                break;
            }
        }
    }
    None
}

fn available_bearings(board_dir: &Path) -> Result<Vec<String>> {
    let bearings_dir = board_dir.join("bearings");
    if !bearings_dir.exists() {
        return Ok(Vec::new());
    }

    let mut bearings: Vec<String> = fs::read_dir(&bearings_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    bearings.sort();

    Ok(bearings)
}

fn fail_unknown_bearing(board_dir: &Path, bearing_id: &str) -> ! {
    eprintln!("Unknown bearing: {}", bearing_id);
    eprintln!("\nAvailable bearings:");

    match available_bearings(board_dir) {
        Ok(bearings) => {
            for bearing in &bearings {
                println!("  {}", bearing);
            }
        }
        Err(err) => {
            eprintln!("Failed to list bearings: {}", err);
        }
    }

    std::process::exit(1);
}

fn load_bearing_brief(board_dir: &Path, bearing_id: &str) -> Result<String> {
    let brief_path = board_dir.join("bearings").join(bearing_id).join("BRIEF.md");
    if !brief_path.exists() {
        fail_unknown_bearing(board_dir, bearing_id);
    }

    fs::read_to_string(&brief_path)
        .with_context(|| format!("Failed to read {}", brief_path.display()))
}

/// Start a freeform play session
fn freeform_play(play_dir: &Path, prop_name: Option<&str>) -> Result<()> {
    if let Some(name) = prop_name {
        return play_with_prop(play_dir, name, None);
    }

    println!("🎭 Ready to play?\n");
    println!("Pick a prop to shift your perspective:\n");

    let props_dir = play_dir.join("props");
    if props_dir.exists() {
        let mut categories: Vec<_> = fs::read_dir(&props_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();
        categories.sort_by_key(|e| e.file_name());

        for category_entry in categories {
            let mut props: Vec<String> = fs::read_dir(category_entry.path())?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
                .map(|e| {
                    e.path()
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                })
                .collect();
            props.sort();

            for prop in &props {
                let prop_path = category_entry.path().join(format!("{}.md", prop));
                let prompt = read_prop_prompt(&prop_path).unwrap_or_default();
                println!("  {:<14} {}", prop, prompt);
            }
        }
    }

    println!("\nStart with:");
    println!("  keel play --prop <name>");
    println!("  keel play <bearing-id>");
    println!("\nOr describe what's fuzzy and we'll find the right prop.");

    Ok(())
}

/// Play with a specific prop, optionally in the context of a bearing
fn play_with_prop(play_dir: &Path, prop_name: &str, bearing_context: Option<&str>) -> Result<()> {
    // Find the prop file
    let props_dir = play_dir.join("props");
    let prop_path = find_prop_file(&props_dir, prop_name);

    let Some(prop_path) = prop_path else {
        eprintln!("Unknown prop: {}", prop_name);
        eprintln!("\nAvailable props:");
        list_available_props(play_dir)?;
        std::process::exit(1);
    };

    let content = fs::read_to_string(&prop_path)?;
    let prompt = read_prop_prompt(&prop_path).unwrap_or_else(|| "...".to_string());

    // Extract the prop name and description
    let title = content
        .lines()
        .find(|l| l.starts_with("# "))
        .unwrap_or("Unknown Prop");

    println!(
        "🎭 Putting on the {} mask\n",
        title.trim_start_matches("# ")
    );
    println!("Core prompt: \"{}\"\n", prompt);

    if let Some(context) = bearing_context {
        println!("Context: {}\n", context);
        println!(
            "Apply the {} lens to this bearing. {} What do you see?\n",
            title.trim_start_matches("# "),
            prompt
        );
    } else {
        println!(
            "What would you like to explore? {} and see where it leads.\n",
            prompt
        );
    }

    // Show the "When to Reach" section as guidance
    let mut in_when = false;
    for line in content.lines() {
        if line.starts_with("## When to Reach") {
            in_when = true;
            println!("Good for:");
            continue;
        }
        if in_when {
            if line.starts_with("## ") {
                break;
            }
            if line.starts_with("- ") {
                println!("  {}", line);
            }
        }
    }

    Ok(())
}

/// Play a bearing — generate a scenario from its BRIEF.md
fn play_bearing(
    board_dir: &Path,
    play_dir: &Path,
    bearing_id: &str,
    prop_name: Option<&str>,
) -> Result<()> {
    let brief = load_bearing_brief(board_dir, bearing_id)?;

    // Extract title and hypothesis from brief
    let title = brief
        .lines()
        .find(|l| l.starts_with("# "))
        .unwrap_or("Unknown Bearing");

    let hypothesis = extract_section(&brief, "## Hypothesis")
        .unwrap_or_else(|| "No hypothesis found.".to_string());

    if let Some(name) = prop_name {
        // Combine prop + bearing context
        println!(
            "🧭 Bearing: {}\n",
            title.trim_start_matches("# ").trim_end_matches(" — Brief")
        );
        println!("Hypothesis: {}\n", hypothesis.trim());
        play_with_prop(play_dir, name, Some(&hypothesis))?;
    } else {
        // Generate a play invitation from the bearing
        println!(
            "🧭 Playing with bearing: {}\n",
            title.trim_start_matches("# ").trim_end_matches(" — Brief")
        );
        println!("{}\n", hypothesis.trim());

        // Show open questions as play prompts
        if let Some(questions) = extract_section(&brief, "## Open Questions") {
            println!("Open questions to play with:");
            for line in questions.lines() {
                if line.starts_with("- ") {
                    println!("  {}", line);
                }
            }
            println!();
        }

        println!("Pick a prop to explore:");
        println!("  keel play {} --prop improviser", bearing_id);
        println!("  keel play {} --prop jester", bearing_id);
        println!("  keel play {} --prop bard", bearing_id);
        println!("  keel play {} --prop playwright", bearing_id);
    }

    Ok(())
}

/// Cross two bearings in one paired play session
fn run_cross(board_dir: &Path, first: &str, second: &str) -> Result<()> {
    if first == second {
        bail!("`--cross` requires two distinct bearing IDs");
    }

    let first_brief = load_bearing_brief(board_dir, first)?;
    let second_brief = load_bearing_brief(board_dir, second)?;

    let first_title = extract_title(&first_brief);
    let second_title = extract_title(&second_brief);

    let first_hypothesis = extract_section(&first_brief, "## Hypothesis")
        .unwrap_or_else(|| "No hypothesis found.".to_string())
        .trim()
        .to_string();
    let second_hypothesis = extract_section(&second_brief, "## Hypothesis")
        .unwrap_or_else(|| "No hypothesis found.".to_string())
        .trim()
        .to_string();

    println!("🧭 Cross-Bearing Play");
    println!(
        "{} — {}\n",
        first_title.trim_end_matches(" — Brief"),
        second_title.trim_end_matches(" — Brief")
    );

    println!("Hypotheses:");
    print_side_by_side(
        "1",
        &first_title,
        &first_hypothesis,
        "2",
        &second_title,
        &second_hypothesis,
    );
    println!();

    let shared_themes = discover_shared_themes(&first_brief, &second_brief);
    if shared_themes.is_empty() {
        println!("🎭 Themes at the junction: (none clearly shared)");
    } else {
        println!("🎭 Shared themes:");
        for theme in &shared_themes {
            println!("  • {}", theme);
        }
    }

    let intersection_mask = suggest_intersection_mask(&shared_themes);
    println!(
        "\n✨ Suggested intersection mask: {}",
        capitalize(&intersection_mask)
    );
    if shared_themes.is_empty() {
        println!(
            "Rationale: These two bearings read like different songs; improvise first to find the bridge."
        );
    } else {
        println!("Rationale: Shared themes suggest this perspective can spotlight the overlap.");
    }

    println!("\n🎭 Bridge prompts:");
    let bridge_prompts = cross_prompts(&first_title, &second_title, &shared_themes);
    for prompt in bridge_prompts {
        println!("  • {}", prompt);
    }

    println!(
        "\nTry next:\n  keel play {} --prop {}\n  keel play {} --prop {}",
        first, intersection_mask, second, intersection_mask
    );

    Ok(())
}

fn print_side_by_side(
    left_label: &str,
    left_title: &str,
    left_body: &str,
    right_label: &str,
    right_title: &str,
    right_body: &str,
) {
    let width = 52;
    let separator = " | ";

    let mut left_lines = Vec::new();
    let mut right_lines = Vec::new();
    left_lines.push(format!(
        "{}) {}",
        left_label,
        left_title.trim_end_matches(" — Brief")
    ));
    right_lines.push(format!(
        "{}) {}",
        right_label,
        right_title.trim_end_matches(" — Brief")
    ));
    left_lines.extend(left_body.lines().map(|line| line.trim().to_string()));
    right_lines.extend(right_body.lines().map(|line| line.trim().to_string()));

    for i in 0..left_lines.len().max(right_lines.len()) {
        let left_part = left_lines.get(i).cloned().unwrap_or_default();
        let right_part = right_lines.get(i).cloned().unwrap_or_default();
        println!("{:<w$}{}{}", left_part, separator, right_part, w = width);
    }
}

fn suggest_intersection_mask(themes: &[String]) -> String {
    if themes.is_empty() {
        return "improviser".to_string();
    }

    let seed = themes.join(" ");
    let ranked = score_masks(&seed);
    if let Some((mask, score, _)) = ranked.first()
        && *score > 0
    {
        return mask.clone();
    }

    "bard".to_string()
}

fn cross_prompts(first: &str, second: &str, themes: &[String]) -> Vec<String> {
    if themes.is_empty() {
        return vec![
            "How would each bearing need to change to avoid canceling each other out?".to_string(),
            "Can one bearing become a rehearsal for the failure mode of the other?".to_string(),
            "What one tiny experiment could advance both at once?".to_string(),
        ];
    }

    let mut prompts = Vec::new();
    for theme in themes.iter().take(2) {
        prompts.push(format!(
            "What does \"{}\" mean differently in {} than in {}?",
            theme, first, second
        ));
        prompts.push(format!(
            "If \"{}\" is true in one bearing, how can the other respond?",
            theme
        ));
    }
    prompts.truncate(4);
    prompts
}

fn discover_shared_themes(left: &str, right: &str) -> Vec<String> {
    let mut left_words = BTreeSet::new();
    let mut right_words = BTreeSet::new();

    let left_sections = [
        extract_section(left, "## Hypothesis"),
        extract_section(left, "## Open Questions"),
        extract_section(left, "## Success Criteria"),
    ];
    let right_sections = [
        extract_section(right, "## Hypothesis"),
        extract_section(right, "## Open Questions"),
        extract_section(right, "## Success Criteria"),
    ];

    for content in left_sections.iter().flatten() {
        for token in extract_keywords(content) {
            if !is_stopword(&token) {
                let _ = left_words.insert(token);
            }
        }
    }
    for content in right_sections.iter().flatten() {
        for token in extract_keywords(content) {
            if !is_stopword(&token) {
                let _ = right_words.insert(token);
            }
        }
    }

    left_words.intersection(&right_words).cloned().collect()
}

fn extract_keywords(text: &str) -> Vec<String> {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .map(str::to_owned)
        .filter(|token| token.len() >= 4)
        .collect()
}

fn is_stopword(token: &str) -> bool {
    const STOP_WORDS: &[&str] = &[
        "a", "about", "after", "again", "against", "all", "almost", "also", "an", "and", "another",
        "any", "are", "around", "as", "at", "away", "because", "been", "before", "being", "below",
        "both", "but", "by", "could", "did", "does", "each", "ever", "few", "for", "from", "had",
        "have", "her", "here", "hers", "his", "how", "into", "is", "it", "its", "just", "more",
        "most", "not", "of", "off", "on", "one", "only", "or", "our", "ours", "over", "same", "so",
        "some", "such", "than", "that", "the", "their", "them", "then", "there", "these", "they",
        "this", "those", "through", "under", "using", "very", "was", "were", "when", "where",
        "which", "while", "with", "without", "would", "you", "your", "yours", "will",
    ];

    STOP_WORDS.binary_search(&token).is_ok()
}

fn extract_title(content: &str) -> String {
    content
        .lines()
        .find(|line| line.starts_with("# "))
        .unwrap_or("Unknown Bearing")
        .trim_start_matches("# ")
        .to_string()
}

/// Find a prop file by name across all categories
fn find_prop_file(props_dir: &Path, name: &str) -> Option<std::path::PathBuf> {
    if !props_dir.exists() {
        return None;
    }

    let mut categories: Vec<_> = fs::read_dir(props_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    categories.sort_by_key(|e| e.file_name());

    for category in categories {
        let prop_path = category.path().join(format!("{}.md", name));
        if prop_path.exists() {
            return Some(prop_path);
        }
    }
    None
}

/// Suggest a mask for a bearing based on its content
fn run_suggest(board_dir: &Path, bearing_id: &str) -> Result<String> {
    let brief = load_bearing_brief(board_dir, bearing_id)?;
    let scores = score_masks(&brief);

    // scores is sorted descending by score
    let (top_mask, top_score, top_rationale) = &scores[0];
    let (runner_up, _, _) = &scores[1];

    let title = brief
        .lines()
        .find(|l| l.starts_with("# "))
        .unwrap_or("Unknown Bearing")
        .trim_start_matches("# ")
        .trim_end_matches(" — Brief");

    println!("Suggesting a mask for: {}\n", title);

    if *top_score == 0 {
        println!("Recommended: Improviser (default — no strong signals detected)");
        println!("Rationale:   When no signal stands out, momentum beats analysis.");
    } else {
        println!("Recommended: {}", capitalize(top_mask));
        println!("Rationale:   {}", top_rationale);
    }

    println!("Runner-up:   {}", capitalize(runner_up));
    println!(
        "\nTry:\n  keel play {} --prop {}",
        bearing_id,
        top_mask.to_lowercase()
    );

    Ok(top_mask.to_lowercase())
}

/// Score each mask based on heuristic signals in the brief content
fn score_masks(brief: &str) -> Vec<(String, i32, String)> {
    let mut improviser_score = 0i32;
    let mut improviser_reasons: Vec<&str> = Vec::new();
    let mut bard_score = 0i32;
    let mut bard_reasons: Vec<&str> = Vec::new();
    let mut playwright_score = 0i32;
    let mut playwright_reasons: Vec<&str> = Vec::new();
    let mut jester_score = 0i32;
    let mut jester_reasons: Vec<&str> = Vec::new();

    let brief_lower = brief.to_lowercase();

    // Signal: Many open questions (5+) → Improviser
    if let Some(questions) = extract_section(brief, "## Open Questions") {
        let question_count = questions.lines().filter(|l| l.starts_with("- ")).count();
        if question_count >= 5 {
            improviser_score += 3;
            improviser_reasons.push("many open questions need momentum, not more analysis");
        } else if question_count >= 3 {
            improviser_score += 1;
            improviser_reasons.push("several open questions to explore");
        }
    }

    // Signal: Narrative/story/why/meaning keywords → Bard
    let bard_keywords = [
        "story",
        "narrative",
        "meaning",
        "why",
        "purpose",
        "emotional",
        "human",
    ];
    let bard_hits: Vec<&&str> = bard_keywords
        .iter()
        .filter(|kw| brief_lower.contains(**kw))
        .collect();
    if bard_hits.len() >= 3 {
        bard_score += 3;
        bard_reasons.push("rich narrative signals — this bearing wants its story told");
    } else if !bard_hits.is_empty() {
        bard_score += 1;
        bard_reasons.push("some narrative threads to pull on");
    }

    // Signal: Tension/conflict/tradeoff keywords → Playwright
    let playwright_keywords = [
        "tension",
        "conflict",
        "tradeoff",
        "trade-off",
        "competing",
        "versus",
        "dilemma",
        "contradiction",
    ];
    let playwright_hits: Vec<&&str> = playwright_keywords
        .iter()
        .filter(|kw| brief_lower.contains(**kw))
        .collect();
    if playwright_hits.len() >= 2 {
        playwright_score += 3;
        playwright_reasons.push("tensions detected — the Playwright can stage the conflict");
    } else if !playwright_hits.is_empty() {
        playwright_score += 1;
        playwright_reasons.push("a tension worth staging");
    }

    // Signal: Stuck/paralysis/stalled keywords → Improviser
    let stuck_keywords = ["stuck", "stall", "paralysis", "blocked", "doldrums"];
    let stuck_hits: Vec<&&str> = stuck_keywords
        .iter()
        .filter(|kw| brief_lower.contains(**kw))
        .collect();
    if !stuck_hits.is_empty() {
        improviser_score += 2;
        improviser_reasons.push("signs of being stuck — momentum will help");
    }

    // Signal: Abstract/system/model keywords → Bard
    let abstract_keywords = ["abstract", "model", "framework", "architecture", "system"];
    let abstract_hits: Vec<&&str> = abstract_keywords
        .iter()
        .filter(|kw| brief_lower.contains(**kw))
        .collect();
    if abstract_hits.len() >= 2 {
        bard_score += 2;
        bard_reasons.push("abstract concepts that need grounding through story");
    }

    // Signal: Parked status or many unchecked criteria → Jester
    if brief_lower.contains("status: parked") {
        jester_score += 3;
        jester_reasons.push("parked bearing — the Jester can name what's really going on");
    }
    if let Some(criteria) = extract_section(brief, "## Success Criteria") {
        let unchecked = criteria.lines().filter(|l| l.contains("- [ ]")).count();
        let checked = criteria.lines().filter(|l| l.contains("- [x]")).count();
        if unchecked > 3 && checked == 0 {
            jester_score += 2;
            jester_reasons
                .push("many unchecked criteria — something unspoken may be blocking progress");
        }
    }

    // Build results with rationale strings
    let mut results = vec![
        (
            "improviser".to_string(),
            improviser_score,
            improviser_reasons.join("; "),
        ),
        ("bard".to_string(), bard_score, bard_reasons.join("; ")),
        (
            "playwright".to_string(),
            playwright_score,
            playwright_reasons.join("; "),
        ),
        (
            "jester".to_string(),
            jester_score,
            jester_reasons.join("; "),
        ),
    ];

    // Sort descending by score. Tiebreaker: improviser > bard > playwright > jester
    // (already in that order, so stable sort preserves it)
    results.sort_by(|a, b| b.1.cmp(&a.1));

    results
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

/// Extract a markdown section by heading
fn extract_section(content: &str, heading: &str) -> Option<String> {
    let mut in_section = false;
    let mut result = String::new();
    let heading_level = heading.chars().take_while(|c| *c == '#').count();

    for line in content.lines() {
        if line.starts_with(heading) {
            in_section = true;
            continue;
        }
        if in_section {
            // Stop at same or higher level heading
            if line.starts_with('#') {
                let level = line.chars().take_while(|c| *c == '#').count();
                if level <= heading_level {
                    break;
                }
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggest_favors_improviser_for_many_open_questions() {
        let brief = r#"# Test — Brief

## Hypothesis

Something to explore.

## Open Questions

- Question one?
- Question two?
- Question three?
- Question four?
- Question five?
- Question six?

## Success Criteria

- [ ] First criterion
"#;
        let scores = score_masks(brief);
        assert_eq!(scores[0].0, "improviser");
        assert!(scores[0].1 > 0);
    }

    #[test]
    fn suggest_favors_bard_for_narrative_content() {
        let brief = r#"# Test — Brief

## Hypothesis

The story of why this system has meaning and purpose.
The narrative thread connects the emotional core to the human experience.

## Open Questions

- One question?
"#;
        let scores = score_masks(brief);
        assert_eq!(scores[0].0, "bard");
        assert!(scores[0].1 > 0);
    }

    #[test]
    fn suggest_favors_playwright_for_tensions() {
        let brief = r#"# Test — Brief

## Hypothesis

There is a fundamental tension between speed and quality.
The tradeoff creates a dilemma: competing approaches with no clear winner.

## Open Questions

- How to resolve?
"#;
        let scores = score_masks(brief);
        assert_eq!(scores[0].0, "playwright");
        assert!(scores[0].1 > 0);
    }

    #[test]
    fn suggest_favors_jester_for_parked_bearing() {
        let brief = r#"---
status: parked
---
# Test — Brief

## Hypothesis

Something that's been sitting here a while.

## Success Criteria

- [ ] Criterion one
- [ ] Criterion two
- [ ] Criterion three
- [ ] Criterion four
"#;
        let scores = score_masks(brief);
        assert_eq!(scores[0].0, "jester");
        assert!(scores[0].1 > 0);
    }

    #[test]
    fn suggest_defaults_to_improviser_on_tie() {
        let brief = r#"# Test — Brief

## Hypothesis

A minimal bearing with no strong signals.
"#;
        let scores = score_masks(brief);
        // All scores should be 0, improviser wins as tiebreaker
        assert_eq!(scores[0].0, "improviser");
        assert_eq!(scores[0].1, 0);
    }

    #[test]
    fn suggest_favors_improviser_for_stuck_signals() {
        let brief = r#"# Test — Brief

## Problem Space

The team is stuck in the doldrums. Analysis paralysis has stalled progress.

## Open Questions

- Why are we blocked?
"#;
        let scores = score_masks(brief);
        assert_eq!(scores[0].0, "improviser");
        assert!(scores[0].1 > 0);
    }

    #[test]
    fn cross_bearing_discovers_shared_themes() {
        let first = r#"# First

## Hypothesis

The pilot can speed up testing.

## Open Questions

- How to measure speed?
- What tradeoff is acceptable?
"#;
        let second = r#"# Second

## Hypothesis

Speed is the same as quality, maybe not.

## Open Questions

- What is the user benefit?
- Should we tradeoff speed for simplicity?
"#;
        let shared = discover_shared_themes(first, second);
        assert!(shared.contains(&"speed".to_string()));
    }

    #[test]
    fn cross_bearing_suggests_mask_for_shared_theme() {
        let mask = suggest_intersection_mask(&[
            "tension".to_string(),
            "tradeoff".to_string(),
            "speed".to_string(),
        ]);
        assert_eq!(mask, "playwright");
    }

    #[test]
    fn cross_bearing_prompts_are_derived_from_themes() {
        let prompts = cross_prompts(
            "First Bear",
            "Second Bear",
            &["speed".to_string(), "quality".to_string()],
        );
        assert_eq!(prompts.len(), 4);
        assert!(prompts[0].contains("What does \"speed\" mean"));
    }
}
