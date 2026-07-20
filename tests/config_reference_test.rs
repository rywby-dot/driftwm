//! Guards `config.reference.toml` against drifting from the compiled-in
//! defaults — it doubles as documentation, so a stale documented default would
//! mislead every user who reads it.
//!
//! Reconstructing the file per its grammar (see
//! `dev/docs/reference-config-format.md`) must yield TOML that parses to exactly
//! `Config::from_toml("")`.

use driftwm::config::Config;
use std::collections::BTreeMap;

const REFERENCE: &str = include_str!("../config.reference.toml");

/// Rebuild a plain TOML config from the reference by uncommenting every
/// documented default and keeping the uncommented `[section]` headers.
fn reconstruct(reference: &str) -> String {
    let mut out = String::new();
    for line in reference.lines() {
        if let Some(rest) = line.strip_prefix("# ") {
            // `# #` introduces prose / an example body — never active config.
            if rest.starts_with('#') {
                continue;
            }
            out.push_str(rest);
            out.push('\n');
        } else if line.starts_with('[') {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// Diff two configs' pretty-Debug output as a line multiset, so HashMap
/// (binding map) ordering doesn't produce spurious differences.
fn debug_diff(reference: &Config, code: &Config) -> String {
    fn line_counts(c: &Config) -> BTreeMap<String, i32> {
        let mut counts = BTreeMap::new();
        for line in format!("{c:#?}").lines() {
            *counts.entry(line.trim().to_string()).or_insert(0) += 1;
        }
        counts
    }
    let (ref_counts, code_counts) = (line_counts(reference), line_counts(code));
    let mut diff = String::new();
    for (line, n) in &ref_counts {
        for _ in 0..(n - code_counts.get(line).copied().unwrap_or(0)) {
            diff.push_str(&format!("  reference-only: {line}\n"));
        }
    }
    for (line, n) in &code_counts {
        for _ in 0..(n - ref_counts.get(line).copied().unwrap_or(0)) {
            diff.push_str(&format!("  code-only:      {line}\n"));
        }
    }
    diff
}

/// `deny_unknown_fields` catches a documented field the code dropped; a warning
/// catches a documented default that violates a clamp or is deprecated.
#[test]
fn reference_reconstruction_parses_without_warnings() {
    let reconstructed = reconstruct(REFERENCE);
    let (_, warnings) = Config::from_toml_collect(&reconstructed).unwrap_or_else(|e| {
        panic!("reconstructed config.reference.toml failed to parse: {e}\n\n{reconstructed}")
    });
    assert!(
        warnings.is_empty(),
        "config.reference.toml documents defaults that warn on parse \
         (out-of-range or deprecated):\n{warnings:#?}"
    );
}

#[test]
fn reference_defaults_match_code_defaults() {
    let reconstructed = reconstruct(REFERENCE);
    let from_reference =
        Config::from_toml(&reconstructed).expect("reconstructed config.reference.toml must parse");
    let from_code = Config::from_toml("").expect("empty config must parse");
    assert!(
        from_reference == from_code,
        "config.reference.toml documents defaults that differ from the code defaults:\n{}",
        debug_diff(&from_reference, &from_code)
    );
}

/// True for a `"combo" = "action"` line, distinguishing real bindings from
/// prose that merely opens with a quoted word.
fn is_binding_line(body: &str) -> bool {
    let Some(rest) = body.strip_prefix('"') else {
        return false;
    };
    let Some(close) = rest.find('"') else {
        return false;
    };
    rest[close + 1..].trim_start().starts_with("= \"")
}

/// Every documented binding — active default or `# #` example — must parse
/// without warnings, so a renamed or removed action lingering in an example
/// surfaces here (a bad action is collected as a warning, not a hard error).
#[test]
fn reference_documented_bindings_parse() {
    let mut by_section: BTreeMap<&str, String> = BTreeMap::new();
    let mut section: Option<&str> = None;
    for line in REFERENCE.lines() {
        if line.starts_with('[') {
            section = Some(line);
            continue;
        }
        // Filters out prose that opens with a quoted word (`"wallpaper", "none"...`),
        // which lacks the `= "` of a real binding's quoted LHS.
        let body = line.trim_start_matches(['#', ' ']);
        if is_binding_line(body)
            && let Some(sec) = section
        {
            let buf = by_section.entry(sec).or_default();
            buf.push_str(body);
            buf.push('\n');
        }
    }

    for (sec, body) in &by_section {
        let toml = format!("{sec}\n{body}");
        let (_, warnings) = Config::from_toml_collect(&toml).unwrap_or_else(|e| {
            panic!("documented bindings under {sec} failed to parse: {e}\n\n{toml}")
        });
        assert!(
            warnings.is_empty(),
            "documented bindings under {sec} produced warnings:\n{warnings:#?}\n\n{toml}"
        );
    }
}

/// The TOML body of each `# # Example[: label]` block: `# #`-prefixed lines
/// running until the next marker, a real blank line, an active default, or a
/// section header.
fn example_blocks(reference: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current: Option<String> = None;
    for line in reference.lines() {
        let is_comment = line.starts_with("# #");
        let is_marker = is_comment && line.trim_start_matches(['#', ' ']).starts_with("Example");
        if is_marker {
            blocks.extend(current.take());
            current = Some(String::new());
        } else if is_comment {
            if let Some(b) = current.as_mut() {
                let toml = line.strip_prefix("# #").unwrap();
                b.push_str(toml.strip_prefix(' ').unwrap_or(toml));
                b.push('\n');
            }
        } else {
            blocks.extend(current.take());
        }
    }
    blocks.extend(current.take());
    blocks
}

/// Every `# # Example:` block that is a complete config fragment (declares
/// `[[window_rules]]` or `[[outputs]]`) must parse without warnings, so the
/// gnarliest snippets (globs, regex, pass_keys, output modes) can't silently
/// drift into invalid config.
#[test]
fn reference_example_blocks_parse() {
    for block in example_blocks(REFERENCE) {
        if !block.contains("[[window_rules]]") && !block.contains("[[outputs]]") {
            continue;
        }
        let (_, warnings) = Config::from_toml_collect(&block)
            .unwrap_or_else(|e| panic!("example block failed to parse: {e}\n\n{block}"));
        assert!(
            warnings.is_empty(),
            "example block produced warnings:\n{warnings:#?}\n\n{block}"
        );
    }
}

/// The reference text from a `# ## <heading>` doc heading up to the next
/// same-level heading (or EOF). The heading is matched only at the start of a
/// line, so a heading string can't accidentally hit a substring inside prose.
fn reference_section(heading: &str) -> &'static str {
    let start = REFERENCE
        .match_indices(heading)
        .map(|(i, _)| i)
        .find(|&i| i == 0 || REFERENCE.as_bytes()[i - 1] == b'\n')
        .unwrap_or_else(|| panic!("config.reference.toml is missing heading {heading:?}"));
    let body = &REFERENCE[start..];
    let end = body[heading.len()..]
        .find("\n# ## ")
        .map_or(body.len(), |i| heading.len() + i);
    &body[..end]
}

/// True if some line of `section` documents `field` as a two-column reference
/// entry: after its `# #` comment prefix and alignment padding, the line begins
/// with `field` and the em-dash separator follows. Requiring the `—` keeps
/// example TOML (`field = value`) and longer field names (`border_color` vs
/// `border_color_focused`) from counting as an entry.
fn section_documents_field(section: &str, field: &str) -> bool {
    section.lines().any(|line| {
        let Some(body) = line.strip_prefix("# #") else {
            return false;
        };
        let Some(rest) = body.trim_start().strip_prefix(field) else {
            return false;
        };
        rest.trim_start().starts_with('—')
    })
}

/// The field names serde lists after "expected one of" in a `deny_unknown_fields`
/// rejection — the backtick-quoted tokens.
fn expected_fields(err: &str) -> Vec<String> {
    let (_, tail) = err
        .split_once("expected one of")
        .unwrap_or_else(|| panic!("not a deny_unknown_fields error:\n{err}"));
    tail.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// Feed a section an unknown field and assert every name serde reports as valid
/// is documented there — the only check that catches a field added in code but
/// forgotten in the docs. The reverse (documented but code-absent) is caught only
/// via example blocks (`reference_example_blocks_parse`); prose-only names aren't verified.
fn assert_all_fields_documented(bogus_toml: &str, heading: &str) {
    let err = Config::from_toml_collect(bogus_toml)
        .expect_err("an unknown field must be rejected by deny_unknown_fields")
        .to_string();
    let fields = expected_fields(&err);
    assert!(!fields.is_empty(), "no field names parsed from:\n{err}");
    let section = reference_section(heading);
    for field in &fields {
        assert!(
            section_documents_field(section, field),
            "field `{field}` exists in code but is undocumented in the \
             {heading:?} section of config.reference.toml"
        );
    }
}

#[test]
fn window_rule_fields_are_all_documented() {
    assert_all_fields_documented(
        "[[window_rules]]\napp_id = \"x\"\nbogus_field_zz = 1\n",
        "# ## Window rules",
    );
}

#[test]
fn output_fields_are_all_documented() {
    assert_all_fields_documented(
        "[[outputs]]\nname = \"eDP-1\"\nbogus_field_zz = 1\n",
        "# ## Outputs",
    );
}

/// The body of every ```` ```toml ```` fence in a markdown document.
fn toml_fences(md: &str) -> Vec<String> {
    let mut fences = Vec::new();
    let mut current: Option<String> = None;
    for line in md.lines() {
        match &mut current {
            Some(buf) if line.trim_start().starts_with("```") => {
                fences.push(std::mem::take(buf));
                current = None;
            }
            Some(buf) => {
                buf.push_str(line);
                buf.push('\n');
            }
            None if line.trim() == "```toml" => current = Some(String::new()),
            None => {}
        }
    }
    fences
}

/// Every window-rule recipe in `docs/window-rules.md` must be valid, warning-free
/// config, so a hand-written recipe can't drift into broken TOML unnoticed.
#[test]
fn window_rules_doc_snippets_parse() {
    const DOC: &str = include_str!("../docs/window-rules.md");
    let mut checked = 0;
    for fence in toml_fences(DOC) {
        if !fence.contains("[[window_rules]]") {
            continue;
        }
        let (_, warnings) = Config::from_toml_collect(&fence)
            .unwrap_or_else(|e| panic!("window-rules.md snippet failed to parse: {e}\n\n{fence}"));
        assert!(
            warnings.is_empty(),
            "window-rules.md snippet produced warnings:\n{warnings:#?}\n\n{fence}"
        );
        checked += 1;
    }
    assert!(
        checked > 0,
        "found no [[window_rules]] snippets in docs/window-rules.md"
    );
}
