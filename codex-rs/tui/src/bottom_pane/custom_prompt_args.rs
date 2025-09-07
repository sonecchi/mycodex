use codex_protocol::custom_prompts::CustomPrompt;

/// Parse the first line of the composer looking for a custom prompt slash form:
/// `/name ARG…` → returns ("name", "ARG…"). Only the first line is considered.
pub(crate) fn parse_slash_and_args(first_line: &str) -> Option<(String, String)> {
    let line = first_line.trim_start();
    let Some(stripped) = line.strip_prefix('/') else {
        return None;
    };
    let rest = stripped.trim_start();
    if rest.is_empty() {
        return None;
    }

    // Find the first whitespace boundary to split name and args.
    let mut split_idx: Option<usize> = None;
    for (i, ch) in rest.char_indices() {
        if ch.is_whitespace() {
            split_idx = Some(i);
            break;
        }
    }

    match split_idx {
        Some(i) => {
            let name = rest[..i].trim();
            if name.is_empty() {
                return None;
            }
            let args = rest[i..].trim();
            Some((name.to_string(), args.to_string()))
        }
        None => {
            // No whitespace after name → no args provided
            let name = rest.trim();
            if name.is_empty() {
                None
            } else {
                Some((name.to_string(), String::new()))
            }
        }
    }
}

/// Expand the custom prompt content by replacing all `{{args}}` occurrences
/// with the provided `args`. If the prompt body does not contain `{{args}}`,
/// the body is returned as-is. Returns None if no prompt with `name` exists.
pub(crate) fn expand_custom_prompt(
    prompts: &[CustomPrompt],
    name: &str,
    args: &str,
) -> Option<String> {
    let prompt = prompts.iter().find(|p| p.name == name)?;
    let body = &prompt.content;
    if body.contains("{{args}}") {
        Some(body.replace("{{args}}", args))
    } else {
        Some(body.clone())
    }
}

