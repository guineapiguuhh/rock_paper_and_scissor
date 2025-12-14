use owo_colors::{ComboColorDisplay, OwoColorize, colors::{Black, Blue, Green, Red, White, Yellow}};

pub fn player_one<T>(input: &T) -> ComboColorDisplay<'_, White, Red, T> {
    input
        .bg::<Red>()
        .fg::<White>()
}

pub fn player_two<T>(input: &T) -> ComboColorDisplay<'_, White, Blue, T> {
    input
        .bg::<Blue>()
        .fg::<White>()
}

pub fn win<T>(input: &T) -> ComboColorDisplay<'_, Black, Yellow, T> {
    input
        .bg::<Yellow>()
        .fg::<Black>()
}

pub fn round<T>(input: &T) -> ComboColorDisplay<'_, Black, Green, T> {
    input
        .bg::<Green>()
        .fg::<Black>()
}