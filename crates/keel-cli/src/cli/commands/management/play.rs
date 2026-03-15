//! `keel play` command — cue the marionette cast for discovery

use std::collections::BTreeSet;
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::Path;

use anyhow::{Context, Result, bail};

use super::play_guidance::{guidance_for_suggest, informational_for_exploration, print_human};
use keel::infrastructure::markdown_sections::extract_section;
use keel::infrastructure::utils::slugify;

/// Run the play command
#[allow(clippy::too_many_arguments)]
pub fn run(
    board_dir: &Path,
    bearing_id: Option<String>,
    prop: Option<String>,
    cross: Option<Vec<String>>,
    list_props: bool,
    suggest: Option<String>,
    theater: bool,
    theme: Option<String>,
    persona: Option<String>,
    mood: Option<String>,
) -> Result<()> {
    let play_dir = board_dir.join("play");

    let guidance = if theater {
        if cross.is_some() {
            bail!("`--theater` cannot be used with `--cross`");
        }
        if list_props {
            bail!("`--theater` cannot be used with `--list-props`");
        }
        if suggest.is_some() {
            bail!("`--theater` cannot be used with `--suggest`");
        }

        launch_theater(
            board_dir,
            &play_dir,
            bearing_id.as_deref(),
            prop.as_deref(),
            theme.as_deref(),
            persona.as_deref(),
            mood.as_deref(),
        )?;
        informational_for_exploration()
    } else if theme.is_some() || persona.is_some() || mood.is_some() {
        bail!("`--theme`, `--persona`, and `--mood` require `--theater`");
    } else if let Some(ids) = cross {
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

const DEFAULT_THEATER_THEME: &str = "drama";
const DEFAULT_THEATER_PERSONA: &str = "neutral";
const DEFAULT_THEATER_MOOD: &str = "adaptive";
const THEATER_THEME_REGISTRY: &[TheaterTheme] = &[
    TheaterTheme {
        id: "action",
        name: "Action",
    },
    TheaterTheme {
        id: "comedy",
        name: "Comedy",
    },
    TheaterTheme {
        id: "drama",
        name: "Drama",
    },
];
const THEATER_PERSONA_REGISTRY: &[TheaterPersona] = &[
    TheaterPersona {
        id: "neutral",
        prompt: "Set the stage and focus on the objective.",
    },
    TheaterPersona {
        id: "standup",
        prompt: "Cue a punchline before the first beat.",
    },
    TheaterPersona {
        id: "shakespeare",
        prompt: "Speak in verse and turn action into metaphor.",
    },
    TheaterPersona {
        id: "broadway",
        prompt: "Stage the scene with flair, spectacle, and timing.",
    },
    TheaterPersona {
        id: "student",
        prompt: "Ask questions to clarify comprehension of the formal rules and constraints.",
    },
    TheaterPersona {
        id: "interrogator",
        prompt: "Identify gaps in evidence and challenge assumptions through aggressive inquiry.",
    },
];
const THEATER_MOOD_REGISTRY: &[TheaterMood] = &[
    TheaterMood {
        id: "adaptive",
        name: "Adaptive",
        prompt_addendum: "Blend clarity and creativity based on the scene pressure.",
        preferred_persona: Some("neutral"),
    },
    TheaterMood {
        id: "playful",
        name: "Playful",
        prompt_addendum: "Favor wit, surprises, and quick pivots that make the session lively.",
        preferred_persona: Some("standup"),
    },
    TheaterMood {
        id: "poetic",
        name: "Poetic",
        prompt_addendum: "Lean on metaphor, cadence, and imagery while keeping action concrete.",
        preferred_persona: Some("shakespeare"),
    },
    TheaterMood {
        id: "spectacular",
        name: "Spectacular",
        prompt_addendum: "Amplify moments with strong imagery, timing, and clear stage transitions.",
        preferred_persona: Some("broadway"),
    },
    TheaterMood {
        id: "focused",
        name: "Focused",
        prompt_addendum: "Keep things direct, structured, and explicit about the next move.",
        preferred_persona: Some("neutral"),
    },
    TheaterMood {
        id: "inquiry",
        name: "Inquiry",
        prompt_addendum: "Turn the session into a formal inquiry of facts, rules, and logic.",
        preferred_persona: Some("student"),
    },
];
const THEATER_PROP_CATEGORIES: &[&str] = &["masks", "hats", "instruments", "costumes", "custom"];

#[derive(Debug, Clone, Copy)]
struct TheaterTheme {
    id: &'static str,
    name: &'static str,
}

struct TheaterPersona {
    id: &'static str,
    prompt: &'static str,
}

struct TheaterMood {
    id: &'static str,
    name: &'static str,
    prompt_addendum: &'static str,
    preferred_persona: Option<&'static str>,
}

fn launch_theater(
    board_dir: &Path,
    play_dir: &Path,
    bearing_id: Option<&str>,
    prop_name: Option<&str>,
    theme: Option<&str>,
    persona: Option<&str>,
    mood: Option<&str>,
) -> Result<()> {
    let selected_theme = resolve_theater_theme(theme)?;
    let selected_mood = resolve_theater_mood(mood);
    let selected_persona = resolve_theater_persona(persona, selected_mood);
    let selected_theme_profile = find_theme_profile(&selected_theme);
    let selected_theme_label = selected_theme_profile
        .map(|profile| profile.name)
        .unwrap_or(selected_theme.as_str());
    let selected_cue = compose_theater_cue(selected_persona, selected_mood);

    println!("🎭 Keel Theater");
    println!("──────────────────────────────");
    println!("Theme:   {} ({})", selected_theme, selected_theme_label);
    println!("Mood:    {} ({})", selected_mood.id, selected_mood.name);
    println!("Persona: {}", selected_persona.id);
    println!(
        "Cue:    [{}:{}] {}",
        selected_theme, selected_persona.id, selected_cue
    );
    println!("Status:  stage is open");
    println!();

    if let Some(bearing) = bearing_id {
        println!("📜 Casting bearing: {}", bearing);
        let brief = load_bearing_brief(board_dir, bearing)?;
        let title = extract_title(&brief);
        println!("Title:   {}", title.trim_end_matches(" — Brief"));
        if let Some(prop) = prop_name {
            play_with_prop(
                play_dir,
                prop,
                Some(&format!(
                    "Apply this to the {} scene as {} mood and {} lens.",
                    selected_theme, selected_mood.id, selected_persona.id
                )),
            )?;
        } else {
            println!("Cue: pick one prop to begin the scene.");
            println!("  keel play --theater {} --prop improviser", bearing);
        }
        return Ok(());
    }

    if let Some(prop) = prop_name {
        play_with_prop(
            play_dir,
            prop,
            Some(&format!(
                "Apply this to a {} scene with {} mood and {} lens.",
                selected_theme, selected_mood.id, selected_persona.id
            )),
        )?;
        return Ok(());
    }

    if let Err(err) = freeform_play(play_dir, None) {
        bail!("failed to start freeform theater session: {}", err);
    }

    Ok(())
}

fn resolve_theater_theme(input: Option<&str>) -> Result<String> {
    let requested = input.unwrap_or(DEFAULT_THEATER_THEME).to_lowercase();
    if find_theme_profile(&requested).is_some() {
        return Ok(requested);
    }

    bail!(
        "Unsupported theme '{}'. Supported themes: {}",
        requested,
        supported_themes()
    );
}

fn find_theme_profile(theme: &str) -> Option<&'static TheaterTheme> {
    let normalized = theme.to_lowercase();
    THEATER_THEME_REGISTRY
        .iter()
        .find(|entry| entry.id == normalized)
}

fn supported_themes() -> String {
    THEATER_THEME_REGISTRY
        .iter()
        .map(|theme| theme.id)
        .collect::<Vec<_>>()
        .join(", ")
}

fn compose_theater_cue(persona: &TheaterPersona, mood: &TheaterMood) -> String {
    if mood.prompt_addendum.is_empty() {
        return persona.prompt.to_string();
    }

    format!(
        "{} {}",
        persona.prompt.trim_end_matches('.'),
        mood.prompt_addendum
    )
}

fn resolve_theater_mood(input: Option<&str>) -> &'static TheaterMood {
    let requested = input.unwrap_or(DEFAULT_THEATER_MOOD).to_lowercase();
    if let Some(profile) = find_mood_profile(&requested) {
        return profile;
    }

    println!(
        "Unknown mood '{}'. Falling back to {}. Supported moods: {}",
        requested,
        DEFAULT_THEATER_MOOD,
        supported_moods()
    );
    &THEATER_MOOD_REGISTRY[0]
}

fn find_mood_profile(id: &str) -> Option<&'static TheaterMood> {
    let normalized = id.to_lowercase();
    THEATER_MOOD_REGISTRY
        .iter()
        .find(|entry| entry.id == normalized)
}

fn supported_moods() -> String {
    THEATER_MOOD_REGISTRY
        .iter()
        .map(|mood| mood.id)
        .collect::<Vec<_>>()
        .join(", ")
}

fn resolve_theater_persona(input: Option<&str>, mood: &TheaterMood) -> &'static TheaterPersona {
    if let Some(persona_input) = input {
        let requested = persona_input.to_lowercase();
        if let Some(profile) = find_persona_profile(&requested) {
            return profile;
        }

        println!(
            "Unknown persona '{}'. Falling back to {}. Supported personas: {}",
            requested,
            DEFAULT_THEATER_PERSONA,
            supported_personas()
        );
    }

    if let Some(mapped_persona) = mood.preferred_persona.and_then(find_persona_profile) {
        return mapped_persona;
    }

    &THEATER_PERSONA_REGISTRY[0]
}

fn find_persona_profile(id: &str) -> Option<&'static TheaterPersona> {
    let normalized = id.to_lowercase();
    THEATER_PERSONA_REGISTRY
        .iter()
        .find(|entry| entry.id == normalized)
}

fn supported_personas() -> String {
    THEATER_PERSONA_REGISTRY
        .iter()
        .map(|persona| persona.id)
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod theater_tests {
    use super::*;

    #[test]
    fn theater_theme_uses_default() {
        let theme = resolve_theater_theme(None).unwrap();
        assert_eq!(theme, DEFAULT_THEATER_THEME);
    }

    #[test]
    fn theater_theme_registry_is_structured() {
        assert!(THEATER_THEME_REGISTRY.len() >= 3);
        assert_eq!(THEATER_THEME_REGISTRY[0].id, "action");
        assert_eq!(THEATER_THEME_REGISTRY[1].id, "comedy");
        assert_eq!(THEATER_THEME_REGISTRY[2].id, "drama");
    }

    #[test]
    fn theater_theme_rejects_unknown() {
        assert!(resolve_theater_theme(Some("opera")).is_err());
    }

    #[test]
    fn theater_mood_registry_is_structured() {
        assert!(THEATER_MOOD_REGISTRY.len() >= 5);
        assert_eq!(THEATER_MOOD_REGISTRY[0].id, "adaptive");
    }

    #[test]
    fn theater_mood_rejects_unknown_to_default() {
        let mood = resolve_theater_mood(Some("absent"));
        assert_eq!(mood.id, DEFAULT_THEATER_MOOD);
    }

    #[test]
    fn theater_persona_defaults_to_neutral() {
        let mood = resolve_theater_mood(Some(DEFAULT_THEATER_MOOD));
        let persona = resolve_theater_persona(None, mood);
        assert_eq!(persona.id, "neutral");
    }

    #[test]
    fn theater_persona_uses_mood_mapping_when_unspecified() {
        let mood = resolve_theater_mood(Some("playful"));
        let persona = resolve_theater_persona(None, mood);
        assert_eq!(persona.id, "standup");
    }

    #[test]
    fn theater_persona_falls_back_on_unknown() {
        let mood = resolve_theater_mood(Some("focused"));
        let persona = resolve_theater_persona(Some("pirate"), mood);
        assert_eq!(persona.id, DEFAULT_THEATER_PERSONA);
    }

    #[test]
    fn theater_personas_have_distinct_templates() {
        assert!(THEATER_PERSONA_REGISTRY.len() >= 4);

        let mut prompts = std::collections::BTreeSet::new();
        for persona in THEATER_PERSONA_REGISTRY {
            assert!(!persona.id.is_empty());
            assert!(!persona.prompt.is_empty());
            prompts.insert(persona.prompt);
        }

        assert!(prompts.len() >= 4);
    }

    #[test]
    fn suggest_similar_props_uses_partial_matches() {
        let available = vec![
            "improviser".to_string(),
            "jester".to_string(),
            "playwright".to_string(),
            "bard".to_string(),
        ];

        let matches = suggest_similar_props("play", &available);
        assert_eq!(matches, vec!["playwright"]);

        let empty = suggest_similar_props("unknown", &available);
        assert!(empty.is_empty());
    }

    #[test]
    fn collect_prop_names_is_stable_for_missing_catalog() {
        let temp = tempfile::tempdir().unwrap();
        let names = collect_prop_names(temp.path()).unwrap();
        assert!(names.is_empty());
    }
}

/// List all available props by category
fn list_available_props(play_dir: &Path) -> Result<()> {
    let props_dir = play_dir.join("props");

    if !props_dir.exists() {
        println!("No props catalog found. Assemble props at .keel/play/props/");
        return Ok(());
    }

    println!("🎭 Marionette props (reframing tools)\n");

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
                "masks" => "Masks (cast personalities)",
                "hats" => "Hats (thinking voices)",
                "instruments" => "Instruments (tempo controls)",
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
        println!(
            "  Empty backstage. Add .md props to .keel/play/props/<category>/ to dress the cast."
        );
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

    println!("🎭 Puppet theater is ready.\n");
    println!("Pull a prop string and shift perspective:\n");

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
    println!("\nOr describe what's fuzzy and we'll find the right string.");

    Ok(())
}

/// Play with a specific prop, optionally in the context of a bearing
fn play_with_prop(play_dir: &Path, prop_name: &str, bearing_context: Option<&str>) -> Result<()> {
    // Find the prop file
    let props_dir = play_dir.join("props");
    let prop_path = find_prop_file(&props_dir, prop_name);

    let Some(prop_path) = prop_path else {
        return handle_unknown_prop(play_dir, prop_name, bearing_context);
    };

    let content = fs::read_to_string(&prop_path)?;
    let prompt = read_prop_prompt(&prop_path).unwrap_or_else(|| "...".to_string());

    // Extract the prop name and description
    let title = content
        .lines()
        .find(|l| l.starts_with("# "))
        .unwrap_or("Unknown Prop");

    println!(
        "🎭 Rigging the {} marionette\n",
        title.trim_start_matches("# ")
    );
    println!("Marionette cue: \"{}\"\n", prompt);

    if let Some(context) = bearing_context {
        println!("Stage:\n{}\n", context);
        println!(
            "Apply the {} lens to this bearing. {} What do you see from the strings?\n",
            title.trim_start_matches("# "),
            prompt
        );
    } else {
        println!(
            "What would you like to explore? {} and follow the strings from there.\n",
            prompt
        );
    }

    // Show the "When to Reach" section as guidance
    let mut in_when = false;
    for line in content.lines() {
        if line.starts_with("## When to Reach") {
            in_when = true;
            println!("Best cue moments:");
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

fn handle_unknown_prop(
    play_dir: &Path,
    prop_name: &str,
    bearing_context: Option<&str>,
) -> Result<()> {
    eprintln!("Unknown prop: {}", prop_name);

    let available = collect_prop_names(play_dir)?;
    if available.is_empty() {
        return handle_unknown_prop_when_catalog_empty(play_dir, prop_name, bearing_context);
    }

    eprintln!("\nAvailable props:");
    for prop in &available {
        println!("  {}", prop);
    }

    let similar = suggest_similar_props(prop_name, &available);
    if !similar.is_empty() {
        println!("\nDid you mean one of these?");
        for prop in similar {
            println!("  {}", prop);
        }
    }

    match prompt_line("Try one of these props (leave blank to start freeform): ") {
        Some(input) if input.is_empty() => freeform_play(play_dir, None),
        Some(input) => {
            let next = input.to_lowercase();
            if let Some(existing) = available
                .iter()
                .find(|name| name.eq_ignore_ascii_case(&next))
            {
                return play_with_prop(play_dir, existing, bearing_context);
            }

            if confirm_prompt(&format!("Create a new prop '{}'? [y/N]: ", input))? {
                return create_named_prop(play_dir, &input, bearing_context);
            }

            eprintln!("No match for '{}'.", input);
            freeform_play(play_dir, None)
        }
        None => {
            eprintln!(
                "Non-interactive mode: pass a known `--prop` or run `keel play --list-props` first."
            );
            bail!("Unknown prop: {prop_name}");
        }
    }
}

fn handle_unknown_prop_when_catalog_empty(
    play_dir: &Path,
    prop_name: &str,
    bearing_context: Option<&str>,
) -> Result<()> {
    eprintln!("No props catalog found. Assemble props at .keel/play/props/");
    if confirm_prompt(&format!(
        "Create and stage a new prop '{}' now? [y/N]: ",
        prop_name
    ))? {
        return create_named_prop(play_dir, prop_name, bearing_context);
    }

    eprintln!(
        "\nStart with an existing prop catalog using `keel play --list-props` for suggestions."
    );
    freeform_play(play_dir, None)
}

fn create_named_prop(
    play_dir: &Path,
    requested_name: &str,
    bearing_context: Option<&str>,
) -> Result<()> {
    let category = prompt_or_default(
        "Select a prop category [masks/hats/instruments/costumes/custom]: ",
        "custom",
    )
    .unwrap_or_else(|| "custom".to_string());
    let normalized_category = THEATER_PROP_CATEGORIES
        .iter()
        .find(|entry| entry.eq_ignore_ascii_case(&category))
        .copied()
        .unwrap_or("custom");
    let prompt = prompt_or_default(
        &format!(
            "Give this prop a one-line prompt (blank for default): [{}] ",
            requested_name
        ),
        &format!("Try a fresh perspective with {}.", requested_name),
    )
    .unwrap_or_else(|| format!("Try a fresh perspective with {}.", requested_name));

    let name = slugify(requested_name);
    let prop_dir = play_dir.join("props").join(normalized_category);
    fs::create_dir_all(&prop_dir)?;

    let mut prop_path = prop_dir.join(format!("{}.md", name));
    if prop_path.exists() {
        let mut suffix = 1usize;
        loop {
            let candidate = prop_dir.join(format!("{}-{}.md", name, suffix));
            if !candidate.exists() {
                prop_path = candidate;
                break;
            }
            suffix += 1;
        }
    }

    write_prop_template(&prop_path, requested_name, &prompt)?;

    let created = prop_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(requested_name);
    play_with_prop(play_dir, created, bearing_context)
}

fn write_prop_template(path: &Path, name: &str, prompt: &str) -> Result<()> {
    let content = format!(
        "# {}\n\n**Reframes by:** {}\n\n## Core Prompt\n> {}\n\n## When to Reach\n- Use this prop when you want a new framing angle.\n- Use it to test assumptions and expose hidden dependencies.\n",
        name, name, prompt
    );
    fs::write(path, content)?;
    Ok(())
}

fn collect_prop_names(play_dir: &Path) -> Result<Vec<String>> {
    let props_dir = play_dir.join("props");
    if !props_dir.exists() {
        return Ok(Vec::new());
    }

    let mut names = Vec::new();
    for entry in fs::read_dir(&props_dir)?.filter_map(Result::ok) {
        if !entry.path().is_dir() {
            continue;
        }
        for prop_entry in fs::read_dir(entry.path())?.filter_map(Result::ok) {
            if prop_entry.path().extension().is_none_or(|ext| ext != "md") {
                continue;
            }
            if let Some(name) = prop_entry.path().file_stem().and_then(|stem| stem.to_str()) {
                names.push(name.to_string());
            }
        }
    }

    names.sort_by_key(|name| name.to_lowercase());
    names.dedup();
    Ok(names)
}

fn suggest_similar_props(requested: &str, available: &[String]) -> Vec<String> {
    let lowered = requested.to_lowercase();
    available
        .iter()
        .filter(|name| {
            let probe = name.to_lowercase();
            probe.contains(&lowered) || lowered.contains(&probe) || probe.starts_with(&lowered)
        })
        .cloned()
        .collect()
}

fn prompt_line(prompt: &str) -> Option<String> {
    if !(io::stdin().is_terminal() && io::stdout().is_terminal()) {
        return None;
    }

    print!("{}", prompt);
    io::stdout().flush().ok()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    Some(input.trim().to_string())
}

fn prompt_or_default(prompt: &str, default: &str) -> Option<String> {
    let response = prompt_line(prompt)?;
    if response.trim().is_empty() {
        Some(default.to_string())
    } else {
        Some(response)
    }
}

fn confirm_prompt(prompt: &str) -> Result<bool> {
    match prompt_line(prompt) {
        Some(response) => {
            let response = response.to_lowercase();
            Ok(matches!(response.as_str(), "y" | "yes"))
        }
        None => Ok(false),
    }
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
            "🧭 Bearing onstage: {}\n",
            title.trim_start_matches("# ").trim_end_matches(" — Brief")
        );
        println!("Hypothesis: {}\n", hypothesis.trim());
        play_with_prop(play_dir, name, Some(&hypothesis))?;
    } else {
        // Generate a play invitation from the bearing
        println!(
            "🧭 Stage direction: {}\n",
            title.trim_start_matches("# ").trim_end_matches(" — Brief")
        );
        println!("{}\n", hypothesis.trim());

        // Show open questions as play prompts
        if let Some(questions) = extract_section(&brief, "## Open Questions") {
            println!("Open questions for the cast:");
            for line in questions.lines() {
                if line.starts_with("- ") {
                    println!("  {}", line);
                }
            }
            println!();
        }

        println!("Select a marionette prop:");
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

    println!("🧭 Double Act: Cross-Bearing Puppetry");
    println!(
        "{} — {}\n",
        first_title.trim_end_matches(" — Brief"),
        second_title.trim_end_matches(" — Brief")
    );

    println!("Act notes:");
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
        println!("🎭 Shared rigging at the junction: (none clearly shared)");
    } else {
        println!("🎭 Shared stage cues:");
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
            "Rationale: These two bearings move on separate clocks; improvise first to find the bridge."
        );
    } else {
        println!("Rationale: Shared cues suggest this perspective can spotlight the overlap.");
    }

    println!("\n🎭 Bridge prompts:");
    let bridge_prompts = cross_prompts(&first_title, &second_title, &shared_themes);
    for prompt in bridge_prompts {
        println!("  • {}", prompt);
    }

    println!(
        "\nTry the next puppet move:\n  keel play {} --prop {}\n  keel play {} --prop {}",
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

    println!("Suggesting a stage mask for: {}\n", title);

    if *top_score == 0 {
        println!("Recommended: Improviser (default — no strong signals detected)");
        println!("Rationale:   When no signal stands out, momentum beats analysis.");
    } else {
        println!("Recommended: {}", capitalize(top_mask));
        println!("Rationale:   {}", top_rationale);
    }

    println!("Runner-up:   {}", capitalize(runner_up));
    println!(
        "\nTry the next puppet move:\n  keel play {} --prop {}",
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
