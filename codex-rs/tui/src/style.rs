use crate::color::blend;
use crate::color::is_light;
use crate::terminal_palette::best_color;
use crate::terminal_palette::default_bg;
use ratatui::style::Color;
use ratatui::style::Style;
use std::env;

pub fn user_message_style() -> Style {
    if prompt_bg_disabled() {
        Style::default()
    } else {
        user_message_style_for(default_bg())
    }
}

/// Returns the style for a user-authored message using the provided terminal background.
pub fn user_message_style_for(terminal_bg: Option<(u8, u8, u8)>) -> Style {
    if prompt_bg_disabled() {
        return Style::default();
    }
    match terminal_bg {
        Some(bg) => Style::default().bg(user_message_bg(bg)),
        None => Style::default(),
    }
}

#[allow(clippy::disallowed_methods)]
pub fn user_message_bg(terminal_bg: (u8, u8, u8)) -> Color {
    let top = if is_light(terminal_bg) {
        (0, 0, 0)
    } else {
        (255, 255, 255)
    };
    best_color(blend(top, terminal_bg, 0.1))
}

fn prompt_bg_disabled() -> bool {
    matches!(
        env::var("CODEX_TUI_DISABLE_PROMPT_BG").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("True")
    )
}
