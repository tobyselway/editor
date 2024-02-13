#[derive(Clone)]
pub struct Config {
    pub font_path: String,
    pub font_size: u16,
    pub line_height: i32,
    pub tab_size: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font_path: "./fonts/ttf/JetBrainsMono-Regular.ttf".into(),
            font_size: 16,
            line_height: 24,
            tab_size: 8,
        }
    }
}

pub trait Configurable {
    fn config(&mut self, config: &Config);
}
