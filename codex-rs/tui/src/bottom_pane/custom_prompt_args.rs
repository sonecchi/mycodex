use codex_protocol::custom_prompts::CustomPrompt;

/// Parse a composer first line that may start with a slash command.
/// Returns (name, args) where name is the token immediately after '/', and
/// args is the remainder of the line after the first whitespace (leading
/// whitespace trimmed). If the line doesn't start with '/', returns None.
pub fn parse_slash_and_args(first_line: &str) -> Option<(String, String)> {
    if !first_line.starts_with('/') {
        return None;
    }
    // Remove leading '/'
    let rest = &first_line[1..];
    let rest = rest.trim_start();
    if rest.is_empty() {
        return None;
    }
    // Split at first whitespace boundary
    let mut iter = rest.char_indices();
    let split_idx = iter.find(|&(_, c)| c.is_whitespace()).map(|(i, _)| i);
    match split_idx {
        Some(idx) => {
            let name = &rest[..idx];
            let args = rest[idx..].trim_start();
            Some((name.to_string(), args.to_string()))
        }
        None => Some((rest.to_string(), String::new())),
    }
}

/// Expand the prompt named `name` by replacing all `{{args}}` occurrences with
/// `args`. Returns None if no matching prompt is found.
#[allow(dead_code)]
pub fn expand_custom_prompt(prompts: &[CustomPrompt], name: &str, args: &str) -> Option<String> {
    prompts
        .iter()
        .find(|p| p.name == name)
        .map(|p| p.content.replace("{{args}}", args))
}
