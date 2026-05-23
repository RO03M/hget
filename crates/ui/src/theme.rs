use gpui::App;
use gpui_component::{Theme, ThemeColor};

pub fn init(cx: &mut App) {
    let json = include_str!("../themes/catppuccin.json");
    let theme_color = serde_json::from_str::<ThemeColor>(json).expect("Failed to load default theme");

    let mut theme = Theme::default();
    theme.colors = theme_color;

    cx.set_global::<Theme>(theme);
}
